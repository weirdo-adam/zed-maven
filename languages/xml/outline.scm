;; Outline shows only structural elements (elements that contain child
;; elements), so leaf elements like <groupId> don't flood the outline.

(Comment) @annotation

(element
  (STag (Name) @name)
  (content
    (element))) @item
