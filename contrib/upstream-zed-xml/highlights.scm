;; XML declaration

(XMLDecl "xml" @keyword)

[ "version" "encoding" "standalone" ] @property

(EncName) @string.special

(VersionNum) @number

[ "yes" "no" ] @boolean

;; Processing instructions

(PI) @embedded

(PI (PITarget) @keyword)

(XmlModelPI "xml-model" @keyword)

(StyleSheetPI "xml-stylesheet" @keyword)

(PseudoAtt (Name) @attribute)

(PseudoAtt (PseudoAttValue) @string)

;; Doctype declaration

(doctypedecl "DOCTYPE" @keyword)

(doctypedecl (Name) @type)

[ "PUBLIC" "SYSTEM" ] @keyword

(PubidLiteral) @string.special

(SystemLiteral (URI) @markup.link)

;; Element declaration (DTD)

(elementdecl
  "ELEMENT" @keyword
  (Name) @tag)

(contentspec
  (_ (Name) @type))

"#PCDATA" @type.builtin

[ "EMPTY" "ANY" ] @string.special.symbol

[ "*" "?" "+" ] @operator

;; Attribute-list declaration (DTD)

(AttlistDecl
  "ATTLIST" @keyword
  (Name) @tag)

(AttDef (Name) @attribute)

(AttDef (Enumeration (Nmtoken) @string))

(DefaultDecl (AttValue) @string)

[
  (StringType)
  (TokenizedType)
] @type.builtin

(NotationType "NOTATION" @type.builtin)

[
  "#REQUIRED"
  "#IMPLIED"
  "#FIXED"
] @attribute

;; Entity declarations (DTD)

(GEDecl
  "ENTITY" @keyword
  (Name) @constant)

(GEDecl (EntityValue) @string)

(NDataDecl
  "NDATA" @keyword
  (Name) @label)

(PEDecl
  "ENTITY" @keyword
  "%" @operator
  (Name) @constant)

(PEDecl (EntityValue) @string)

;; Notation declaration (DTD)

(NotationDecl
  "NOTATION" @keyword
  (Name) @constant)

(NotationDecl
  (ExternalID
    (SystemLiteral (URI) @string.special)))

;; Entities & references

(EntityRef) @constant

((EntityRef) @constant.builtin
 (#any-of? @constant.builtin
   "&amp;" "&lt;" "&gt;" "&quot;" "&apos;"))

(CharRef) @constant

(PEReference) @constant

;; Tags

(STag (Name) @tag)

(ETag (Name) @tag)

(EmptyElemTag (Name) @tag)

;; Attributes

(Attribute (Name) @attribute)

(Attribute (AttValue) @string)

;; Delimiters & punctuation

[
  "<?" "?>"
  "<!" "]]>"
  "<" ">"
  "</" "/>"
] @punctuation.delimiter

[ "(" ")" "[" "]" ] @punctuation.bracket

[ "\"" "'" ] @punctuation.delimiter

[ "," "|" "=" ] @operator

;; CDATA sections

(CDSect
  (CDStart) @markup.heading)

(CDSect
  (CData) @markup.raw)

(CDSect
  "]]>" @markup.heading)

;; Comments

(Comment) @comment

(ERROR) @error
