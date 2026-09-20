#!/usr/bin/env python3
"""LSP stdio smoke test for LemMinX + lemminx-maven packaging.

Validates that a `lemminx-uber.jar` + `lemminx-maven-deps.zip` pair can be
launched together, that the Maven extension activates, and that dependency
completion works against the local ~/.m2 repository.

Usage:
    python3 scripts/smoke.py --java <java-bin> \
        --lemminx-jar <path/to/lemminx-uber.jar> \
        --ext-zip <path/to/lemminx-maven-deps.zip>
"""

import argparse
import json
import os
import queue
import subprocess
import sys
import tempfile
import threading
import time
import zipfile

FIXTURE_POM = """<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0">
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>smoke</artifactId>
    <version>0.0.1-SNAPSHOT</version>
    <dependencies>
        <dependency>
            <groupId></groupId>
            <artifactId></artifactId>
        </dependency>
    </dependencies>
</project>
"""


def parse_args():
    p = argparse.ArgumentParser()
    p.add_argument("--java", default="java")
    p.add_argument("--lemminx-jar", required=True)
    p.add_argument("--ext-zip", required=True, help="zip of lemminx-maven + dependency jars")
    p.add_argument("--warmup", type=float, default=10.0)
    p.add_argument("--timeout", type=float, default=120.0)
    return p.parse_args()


class LspClient:
    def __init__(self, proc):
        self.proc = proc
        self.msgs = queue.Queue()
        self._id = 0
        threading.Thread(target=self._reader, daemon=True).start()

    def _reader(self):
        f = self.proc.stdout
        while True:
            headers = {}
            while True:
                line = f.readline()
                if not line:
                    return
                line = line.strip()
                if not line:
                    break
                k, _, v = line.partition(b":")
                headers[k.strip()] = v.strip()
            n = int(headers[b"Content-Length"])
            body = b""
            while len(body) < n:
                chunk = f.read(n - len(body))
                if not chunk:
                    return
                body += chunk
            self.msgs.put(json.loads(body))

    def send(self, method, params, has_id=True):
        msg = {"jsonrpc": "2.0", "method": method, "params": params}
        if has_id:
            self._id += 1
            msg["id"] = self._id
        body = json.dumps(msg).encode()
        self.proc.stdin.write(f"Content-Length: {len(body)}\r\n\r\n".encode() + body)
        self.proc.stdin.flush()

    def wait_for(self, pred, timeout=60):
        deadline = time.time() + timeout
        notifications = []
        while time.time() < deadline:
            try:
                m = self.msgs.get(timeout=1)
            except queue.Empty:
                continue
            if pred(m):
                return m, notifications
            notifications.append(m)
        return None, notifications


def completion_items(result):
    if result is None:
        return []
    if isinstance(result, list):
        return result
    return result.get("items", [])


def main():
    args = parse_args()
    lemminx_jar = os.path.abspath(args.lemminx_jar)

    workdir = tempfile.mkdtemp(prefix="lemminx-smoke-")
    ext_dir = os.path.join(workdir, "maven-ext")
    os.makedirs(ext_dir)
    with zipfile.ZipFile(args.ext_zip) as z:
        z.extractall(ext_dir)
    jar_count = len([n for n in os.listdir(ext_dir) if n.endswith(".jar")])
    print(f"[setup] extracted {jar_count} jars into {ext_dir}")
    if jar_count == 0:
        print("[FAIL] extension zip contains no jars")
        return 1

    project = os.path.join(workdir, "project")
    os.makedirs(project)
    pom = os.path.join(project, "pom.xml")
    with open(pom, "w") as f:
        f.write(FIXTURE_POM)
    lines = FIXTURE_POM.split("\n")

    log_path = os.path.join(workdir, "server.log")
    logf = open(log_path, "wb")
    proc = subprocess.Popen(
        [args.java, "-cp", f"{lemminx_jar}:{ext_dir}/*",
         "org.eclipse.lemminx.XMLServerLauncher"],
        stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    threading.Thread(
        target=lambda: [logf.write(c) for c in iter(lambda: proc.stderr.read1(4096), b"")],
        daemon=True).start()

    ok = True
    try:
        client = LspClient(proc)
        client.send("initialize", {
            "processId": os.getpid(),
            "rootUri": "file://" + project,
            "workspaceFolders": [{"uri": "file://" + project, "name": "project"}],
            "capabilities": {
                "workspace": {"didChangeWatchedFiles": {"dynamicRegistration": True},
                              "configuration": True},
                "textDocument": {"completion": {"completionItem": {"snippetSupport": True}}},
            },
        })
        init, _ = client.wait_for(lambda m: m.get("id") == 1, timeout=args.timeout)
        if not init:
            print("[FAIL] no initialize response")
            return 1
        print("[ok] initialize")

        client.send("initialized", {}, has_id=False)
        client.send("textDocument/didOpen", {
            "textDocument": {"uri": "file://" + pom, "languageId": "xml",
                             "version": 1, "text": FIXTURE_POM}}, has_id=False)
        time.sleep(args.warmup)

        results = {}
        for name, line_key in [("groupId", "<groupId></groupId>"),
                               ("artifactId", "<artifactId></artifactId>")]:
            ln = next(i for i, l in enumerate(lines) if line_key in l)
            col = lines[ln].index(line_key) + len(line_key.split(">")[0]) + 1
            client.send("textDocument/completion", {
                "textDocument": {"uri": "file://" + pom},
                "position": {"line": ln, "character": col}})
            comp, _ = client.wait_for(lambda m: m.get("id") == client._id, timeout=args.timeout)
            items = completion_items(comp.get("result")) if comp else []
            results[name] = items
            print(f"[{'ok' if items else 'FAIL'}] {name} completion: {len(items)} items")
            if not items:
                print("       raw:", json.dumps(comp)[:500] if comp else "<no response>")
            for it in items[:3]:
                print("       -", it.get("label", "")[:70])
            if not items:
                ok = False

        client.send("shutdown", None)
        client.wait_for(lambda m: m.get("id") == client._id, timeout=10)
        client.send("exit", None, has_id=False)
        try:
            proc.wait(timeout=15)
        except subprocess.TimeoutExpired:
            proc.kill()
    finally:
        time.sleep(0.3)
        logf.flush()
        with open(log_path, errors="replace") as f:
            errors = [l for l in f if "SEVERE" in l or "严重" in l]
        if errors:
            ok = False
            print(f"[FAIL] {len(errors)} severe log lines (see {log_path}):")
            for l in errors[:10]:
                print("       ", l.rstrip()[:160])
    print("SMOKE:", "PASS" if ok else "FAIL")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
