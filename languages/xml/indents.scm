;; Indent the content between <tag> ... </tag>

(element
  (STag) @start
  (ETag) @end) @indent

;; Indent the internal subset of <!DOCTYPE ... [ ... ]>

(doctypedecl
  "[" @start
  "]" @end) @indent
