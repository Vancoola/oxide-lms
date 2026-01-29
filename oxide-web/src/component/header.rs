use leptos::prelude::{component, view, IntoView, ElementChild};

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header>
            <nav class="flex flex-row justify-between items-center w-full px-4 p-4 border-b border-gray-300">
              <a href="/" class="text-xl font-bold">Digital University Oxide</a>
              <div class="flex-1 flex justify-center">
                <ul class="flex flex-row gap-4 p-0 m-0 list-none">
                  <li><a href="/programs" class="hover:text-gray-600">Academic Programs</a></li>
                  <li><a href="#" class="hover:text-gray-600">Student Experience</a></li>
                  <li><a href="#" class="hover:text-gray-600">Become a Student</a></li>
                  <li><a href="#" class="hover:text-gray-600">Admissions</a></li>
                  <li><a href="#" class="hover:text-gray-600">About</a></li>
                </ul>
              </div>
              <div class="">
                <button class="px-4 py-2 bg-blue-500 text-white font-semibold rounded-lg shadow-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-400 focus:ring-opacity-75">
                    Log in</button>
              </div>
            </nav>
        </header>
    }
}