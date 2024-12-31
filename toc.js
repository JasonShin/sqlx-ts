// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="index.html">Introduction</a></li><li class="chapter-item expanded affix "><li class="part-title">Guides</li><li class="chapter-item expanded "><a href="guides/0.introduction.html"><strong aria-hidden="true">1.</strong> Introduction</a></li><li class="chapter-item expanded "><a href="guides/1.getting-started.html"><strong aria-hidden="true">2.</strong> Getting started</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="guides/1.1.install_sh_cli_options.html"><strong aria-hidden="true">2.1.</strong> curl approach</a></li></ol></li><li class="chapter-item expanded "><a href="guides/2.configuration.html"><strong aria-hidden="true">3.</strong> Configuration</a></li><li class="chapter-item expanded affix "><li class="part-title">API</li><li class="chapter-item expanded "><a href="api/1.connecting-to-db.html"><strong aria-hidden="true">4.</strong> Connecting to databases</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="api/1.3.configs-file-based.html"><strong aria-hidden="true">4.1.</strong> .sqlxrc.json file</a></li><li class="chapter-item expanded "><a href="api/1.1.cli-options.html"><strong aria-hidden="true">4.2.</strong> CLI options</a></li><li class="chapter-item expanded "><a href="api/1.2.environment-variables.html"><strong aria-hidden="true">4.3.</strong> environment variables</a></li></ol></li><li class="chapter-item expanded "><a href="api/2.ignore-patterns.html"><strong aria-hidden="true">5.</strong> .sqlxignore file</a></li><li class="chapter-item expanded affix "><li class="part-title">Reference Guide</li><li class="chapter-item expanded "><a href="reference-guide/1.sql-check.html"><strong aria-hidden="true">6.</strong> SQL Check</a></li><li class="chapter-item expanded "><a href="reference-guide/4.typescript-types-generation.html"><strong aria-hidden="true">7.</strong> Type generation</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="reference-guide/4.1.SELECT.html"><strong aria-hidden="true">7.1.</strong> SELECT</a></li><li class="chapter-item expanded "><a href="reference-guide/4.2.INSERT.html"><strong aria-hidden="true">7.2.</strong> INSERT</a></li><li class="chapter-item expanded "><a href="reference-guide/4.3.DELETE.html"><strong aria-hidden="true">7.3.</strong> DELETE</a></li><li class="chapter-item expanded "><a href="reference-guide/4.4.UPDATE.html"><strong aria-hidden="true">7.4.</strong> UPDATE</a></li><li class="chapter-item expanded "><a href="reference-guide/4.5.annotations.html"><strong aria-hidden="true">7.5.</strong> Annotations</a></li></ol></li><li class="chapter-item expanded "><a href="reference-guide/5.errors.html"><strong aria-hidden="true">8.</strong> Errors</a></li><li class="chapter-item expanded affix "><li class="part-title">Miscellaneous</li><li class="chapter-item expanded "><a href="misc/1.troubleshooting.html"><strong aria-hidden="true">9.</strong> Troubleshooting</a></li><li class="chapter-item expanded "><a href="misc/2.limitations.html"><strong aria-hidden="true">10.</strong> Limitations</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString();
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
