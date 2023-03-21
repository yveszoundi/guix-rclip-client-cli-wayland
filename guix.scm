(define-module (gnu packages rclip-client-cli)
  #:use-module (gnu packages crates-io)
  #:use-module (gnu packages crates-graphics)
  #:use-module (gnu packages tls)
  #:use-module (gnu packages pkg-config)
  #:use-module (guix packages)
  #:use-module (guix download)
  #:use-module (guix git-download)
  #:use-module (guix build-system cargo)
  #:use-module (guix build utils)
  #:use-module (ice-9 popen)
  #:use-module (ice-9 rdelim)
  #:use-module (guix gexp)
  #:use-module ((guix licenses) #:prefix license:))

(define %source-dir (dirname (current-filename)))

(define (git-output . args)
  "Execute 'git ARGS ...' command and return its output without trailing
newspace."
  (with-directory-excursion %source-dir
                            (let* ((port   (apply open-pipe* OPEN_READ "git" args))
                                   (output (read-string port)))
                              (close-port port)
                              (string-trim-right output #\newline))))

(define (current-commit)
  (git-output "log" "-n" "1" "--pretty=format:%H"))

(define-public rclip-client-cli
  (package
   (name "rclip-client-cli")
   (version (string-append "1.0.3" "-" (string-take (current-commit) 7)))
   (source (local-file %source-dir
                       #:recursive? #t
                       #:select? (git-predicate %source-dir)))
   (build-system cargo-build-system)
   (arguments
    `(#:cargo-inputs
      (("rust-clap"            ,rust-clap-3)
       ("rust-nix"             ,rust-nix-0.26)
       ("rust-rustls"          ,rust-rustls-0.20)
       ("rust-wayland-sys"     ,rust-wayland-sys-0.28)
       ("rust-wayland-scanner" ,rust-wayland-scanner-0.28)
       ("rust-dirs"            ,rust-dirs-4)
       ("rust-xml-rs"          ,rust-xml-rs-0.8)
       ("rust-wl-clipboard-rs" ,rust-wl-clipboard-rs-0.4)
       ("rust-serde"           ,rust-serde-1)
       ("rust-serde-derive"    ,rust-serde-derive-1)
       ("rust-toml"            ,rust-toml-0.5))))
   (native-inputs
    `(("pkg-config" ,pkg-config)))
   (inputs
    `(("openssl" ,openssl)))
   (home-page
    "https://github.com/yveszoundi/guix-rclip-client-cli-wayland")
   (synopsis
    "Share clipboard text over a network.")
   (description
    "Simple clipboard utility for sharing text over a network.")
   (license license:gpl3)))

rclip-client-cli
