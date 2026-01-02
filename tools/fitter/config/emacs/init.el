;; ;; org directory
;; (setq org-directory "~/Documents/Emacs/org")
;; 
;; ;; mobile-org directory
;; (setq org-mobile-directory (expand-file-name "~/Dropbox/Apps/MobileOrg"))
;; 
;; ;; 12 hours format
;; (setq org-timestamp-12-hours t)
;; 
;; ;; hide all non scheduled items from agenda view
;; (setq org-hide-all-non-scheduled-items t)
;; 
;; ;; don's source the tags/context from the context.txt file
;; (setq org-disable-context-file t)
;; 
;; ;; practical.org.el GTD+Zettelkasten workflow
;; (setq practical-org-mode-el
;;       (expand-file-name "~/.emacs.d/pkgs/practical.org.el/practical.org.el"))
;; (load-file practical-org-mode-el)

(require 'org)

;; Files
(setq org-directory "~/org")
(setq org-agenda-files (list "inbox.org" "agenda.org"
                             "notes.org" "projects.org"))

;; Capture
(setq org-capture-templates
      `(("i" "Inbox" entry  (file "inbox.org")
        ,(concat "* TODO %?\n"
                 "/Entered on/ %U"))
        ("m" "Meeting" entry  (file+headline "agenda.org" "Future")
        ,(concat "* %? :meeting:\n"
                 "<%<%Y-%m-%d %a %H:00>>"))
        ("n" "Note" entry  (file "notes.org")
        ,(concat "* Note (%a)\n"
                 "/Entered on/ %U\n" "\n" "%?"))
        ("@" "Inbox [mu4e]" entry (file "inbox.org")
        ,(concat "* TODO Reply to \"%a\" %?\n"
                 "/Entered on/ %U"))))

(defun org-capture-inbox ()
     (interactive)
     (call-interactively 'org-store-link)
     (org-capture nil "i"))

(defun org-capture-mail ()
  (interactive)
  (call-interactively 'org-store-link)
  (org-capture nil "@"))

;; Use full window for org-capture
(add-hook 'org-capture-mode-hook 'delete-other-windows)

;; Key bindings
(define-key global-map            (kbd "C-c a") 'org-agenda)
(define-key global-map            (kbd "C-c c") 'org-capture)
(define-key global-map            (kbd "C-c i") 'org-capture-inbox)

;; Only if you use mu4e
;; (require 'mu4e)
;; (define-key mu4e-headers-mode-map (kbd "C-c i") 'org-capture-mail)
;; (define-key mu4e-view-mode-map    (kbd "C-c i") 'org-capture-mail)

;; Refile
(setq org-refile-use-outline-path 'file)
(setq org-outline-path-complete-in-steps nil)
(setq org-refile-targets
      '(("projects.org" :regexp . "\\(?:\\(?:Note\\|Task\\)s\\)")))

;; TODO
(setq org-todo-keywords
      '((sequence "TODO(t)" "NEXT(n)" "HOLD(h)" "|" "DONE(d)")))
(defun log-todo-next-creation-date (&rest ignore)
  "Log NEXT creation time in the property drawer under the key 'ACTIVATED'"
  (when (and (string= (org-get-todo-state) "NEXT")
             (not (org-entry-get nil "ACTIVATED")))
    (org-entry-put nil "ACTIVATED" (format-time-string "[%Y-%m-%d]"))))
(add-hook 'org-after-todo-state-change-hook #'log-todo-next-creation-date)

;; Agenda
(setq org-agenda-custom-commands
      '(("g" "Get Things Done (GTD)"
         ((agenda ""
                  ((org-agenda-skip-function
                    '(org-agenda-skip-entry-if 'deadline))
                   (org-deadline-warning-days 0)))
          (todo "NEXT"
                ((org-agenda-skip-function
                  '(org-agenda-skip-entry-if 'deadline))
                 (org-agenda-prefix-format "  %i %-12:c [%e] ")
                 (org-agenda-overriding-header "\nTasks\n")))
          (agenda nil
                  ((org-agenda-entry-types '(:deadline))
                   (org-agenda-format-date "")
                   (org-deadline-warning-days 7)
                   (org-agenda-skip-function
                    '(org-agenda-skip-entry-if 'notregexp "\\* NEXT"))
                   (org-agenda-overriding-header "\nDeadlines")))
          (tags-todo "inbox"
                     ((org-agenda-prefix-format "  %?-12t% s")
                      (org-agenda-overriding-header "\nInbox\n")))
          (tags "CLOSED>=\"<today>\""
                ((org-agenda-overriding-header "\nCompleted today\n")))))))

(require 'org-tempo)

;; 默认的结构模板映射
;; (setq org-structure-template-alist
;;       '(("a" . "export ascii")
;;         ("c" . "center")
;;         ("C" . "comment")
;;         ("e" . "example")     ; <-- 这就是 <e 的来源
;;         ("E" . "export")
;;         ("h" . "export html")
;;         ("l" . "export latex")
;;         ("q" . "quote")
;;         ("s" . "src")         ; 源代码块
;;         ("v" . "verse")))

;; (use-package org
;;   :config
;;   (setq org-structure-template-alist
;;         '(
;;           ("a" . "export ascii")
;;           ("c" . "center")
;;           ("C" . "comment")
;;           ("e" . "example")
;;           ("E" . "export")
;;           ("h" . "export html")
;;           ("l" . "export latex")
;;           ("q" . "quote")
;;           ("s" . "src")
;;           ("v" . "verse")
;;           ;; 扩展更多模板
;;           ("S" . "src")
;;           ("el" . "src emacs-lisp")
;;           ("py" . "src python")
;;           ("sh" . "src shell")
;;           ("js" . "src javascript")
;;           )))

;; (with-eval-after-load 'org
;;   (setq org-structure-template-alist
;;         '(("a" . "export ascii")
;;           ("c" . "center")
;;           ("C" . "comment")
;;           ("e" . "example")
;;           ("E" . "export")
;;           ("h" . "export html")
;;           ("l" . "export latex")
;;           ("q" . "quote")
;;           ("s" . "src")
;;           ("v" . "verse"))))

;; ;; 清除现有配置（如果需要）
;; (setq org-structure-template-alist nil)
;; 
;; ;; 逐一添加
;; (add-to-list 'org-structure-template-alist '("a" . "export ascii"))
;; (add-to-list 'org-structure-template-alist '("c" . "center"))
;; (add-to-list 'org-structure-template-alist '("C" . "comment"))
;; (add-to-list 'org-structure-template-alist '("e" . "example"))
;; (add-to-list 'org-structure-template-alist '("E" . "export"))
;; (add-to-list 'org-structure-template-alist '("h" . "export html"))
;; (add-to-list 'org-structure-template-alist '("l" . "export latex"))
;; (add-to-list 'org-structure-template-alist '("q" . "quote"))
;; (add-to-list 'org-structure-template-alist '("s" . "src"))
;; (add-to-list 'org-structure-template-alist '("v" . "verse"))
