use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    // let (count, set_count) = create_signal(0);
    // let on_click = move |_| set_count.update(|count| *count += 1);

    let items: Vec<&str> = vec!["Button Link", "Padding", "Border", "Responsive Options"];

    view! {
        <div class="flex flex-col items-center justify-start h-screen w-full bg-gray-200">
            <div class="flex flex-row items-center justify-between w-full bg-gray-800 border-b border-b-gray-950 h-16 px-4">
                <h1 class="text-gray-300 text-xl font-semibold">"Welcome to Leptos! This is Chatbot."</h1>
            </div>
            <div class="flex flex-row grow w-full">
                <div class="w-80 bg-gray-800"></div>
                <div class="flex flex-col p-8">
                    // <button class="text-gray-300 bg-gray-700 hover:bg-gray-800 rounded-lg px-4 py-2 border-2 border-amber-700" on:click=on_click>"Click Me: " {count}</button>
                    <div class="flex flex-row w-[800px] h-[460px] bg-gray-700 rounded-xl shadow-xl shadow-gray-800 overflow-hidden">
                        <div class="flex flex-col w-64 border-r border-gray-900">
                            {items.iter().map(|item| {
                                let strItem = item.to_string();
                                view! {
                                    <div class="flex flex-row items-center justify-start px-4 py-2 border-b border-gray-900 hover:bg-gray-800 cursor-pointer">
                                        <div class="text-gray-400">{strItem}</div>
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>  
                </div>
            </div>
        </div>
    }
}
