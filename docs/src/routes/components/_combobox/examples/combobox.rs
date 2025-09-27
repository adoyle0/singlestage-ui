use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn ComboboxExample() -> impl IntoView {
    view! {
        "A combobox is just a select with a popover inside"
        <ComboBox>
            <button
                type="button"
                class="btn-outline justify-between font-normal w-[200px]"
                id="select-909078-trigger"
                aria-haspopup="listbox"
                aria-expanded="false"
                aria-controls="select-909078-listbox"
            >
                <span class="truncate">"Hey"</span>

                {icon!(icondata::LuChevronsUpDown)}
            </button>
            <div id="select-909078-popover" data-popover aria-hidden="true">
                <header>
                    {icon!(icondata::LuSearch)}
                    <input
                        type="text"
                        value=""
                        placeholder="Search framework..."
                        autocomplete="off"
                        autocorrect="off"
                        spellcheck="false"
                        aria-autocomplete="list"
                        role="combobox"
                        aria-expanded="false"
                        aria-controls="select-909078-listbox"
                        aria-labelledby="select-909078-trigger"
                    />
                </header>

                <div
                    role="listbox"
                    id="select-909078-listbox"
                    aria-orientation="vertical"
                    aria-labelledby="select-909078-trigger"
                    data-empty="No framework found."
                >
                    <div role="option" data-value="Next.js">
                        "Next.js"
                    </div>

                    <div role="option" data-value="SvelteKit">
                        "SvelteKit"
                    </div>

                    <div role="option" data-value="Nuxt.js">
                        "Nuxt.js"
                    </div>

                    <div role="option" data-value="Remix">
                        "Remix"
                    </div>

                    <div role="option" data-value="Astro">
                        "Astro"
                    </div>
                </div>
            </div>
            <input type="hidden" name="select-909078-value" value="" />
        </ComboBox>
    }
}
