use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ToastExample() -> impl IntoView {
    view! {
        <Button
            variant="outline"
            attr:onclick="document.dispatchEvent(new CustomEvent('basecoat:toast', {
            detail: {
            config: {
            category: 'success',
            title: 'Success',
            description: 'A success toast called from the front-end.',
            cancel: {
            label: 'Dismiss'
            }
            }
            }
            }))"
        >
            "Toast from front-end"
        </Button>
        <div id="toaster" class="singlestage-toaster">
            <div
                class="singlestage-toast"
                role="status"
                aria-atomic="true"
                aria-hidden="false"
                data-category="success"
            >
                <div class="singlestage-toast-content">
                    <svg
                        aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg"
                        width="24"
                        height="24"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    >
                        <circle cx="12" cy="12" r="10" />
                        <path d="m9 12 2 2 4-4" />
                    </svg>

                    <section>
                        <h2>"Success"</h2>
                        <p>"A success toast called from the front-end."</p>
                    </section>

                    <footer>
                        <button type="button" class="singlestage-btn" data-toast-action>
                            "Dismiss"
                        </button>
                    </footer>
                </div>
            </div>
        </div>
    }
}
