use leptos::prelude::*;

#[component]
pub fn Stats() -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
                <div class="bg-white p-5 rounded-2xl shadow-soft flex flex-col justify-between h-40 relative overflow-hidden group hover:shadow-lg transition-shadow">
                    <div class="flex justify-between items-start z-10">
                        <div>
                            <p class="text-gray-500 text-sm font-medium mb-1">Total Students</p>
                            <h3 class="text-3xl font-bold text-gray-900">12,450</h3>
                        </div>
                        <div class="w-10 h-10 rounded-full bg-primary-light flex items-center justify-center text-primary">
                            <i data-fa-i2svg=""><svg class="svg-inline--fa fa-user-graduate" aria-hidden="true" focusable="false" data-prefix="fas" data-icon="user-graduate" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" data-fa-i2svg=""><path fill="currentColor" d="M219.3 .5c3.1-.6 6.3-.6 9.4 0l200 40C439.9 42.7 448 52.6 448 64s-8.1 21.3-19.3 23.5L352 102.9V160c0 70.7-57.3 128-128 128s-128-57.3-128-128V102.9L48 93.3v65.1l15.7 78.4c.9 4.7-.3 9.6-3.3 13.3s-7.6 5.9-12.4 5.9H16c-4.8 0-9.3-2.1-12.4-5.9s-4.3-8.6-3.3-13.3L16 158.4V86.6C6.5 83.3 0 74.3 0 64C0 52.6 8.1 42.7 19.3 40.5l200-40zM111.9 327.7c10.5-3.4 21.8 .4 29.4 8.5l71 75.5c6.3 6.7 17 6.7 23.3 0l71-75.5c7.6-8.1 18.9-11.9 29.4-8.5C401 348.6 448 409.4 448 481.3c0 17-13.8 30.7-30.7 30.7H30.7C13.8 512 0 498.2 0 481.3c0-71.9 47-132.7 111.9-153.6z"></path></svg></i>
                        </div>
                    </div>
                    <div class="z-10 mt-auto">
                        <div class="flex items-center text-xs font-medium text-green-500 bg-green-50 w-fit px-2 py-1 rounded-md">
                            <i class="mr-1" data-fa-i2svg=""><svg class="svg-inline--fa fa-arrow-trend-up" aria-hidden="true" focusable="false" data-prefix="fas" data-icon="arrow-trend-up" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512" data-fa-i2svg=""><path fill="currentColor" d="M384 160c-17.7 0-32-14.3-32-32s14.3-32 32-32H544c17.7 0 32 14.3 32 32V288c0 17.7-14.3 32-32 32s-32-14.3-32-32V205.3L342.6 374.6c-12.5 12.5-32.8 12.5-45.3 0L192 269.3 54.6 406.6c-12.5 12.5-32.8 12.5-45.3 0s-12.5-32.8 0-45.3l160-160c12.5-12.5 32.8-12.5 45.3 0L320 306.7 466.7 160H384z"></path></svg></i>
                            <span>+12% this month</span>
                        </div>
                    </div>
                    <div class="absolute -bottom-4 -right-4 w-24 h-24 bg-orange-50 rounded-full opacity-50 group-hover:scale-110 transition-transform"></div>
                </div>

                <div class="bg-white p-5 rounded-2xl shadow-soft flex flex-col justify-between h-40 relative overflow-hidden group hover:shadow-lg transition-shadow">
                    <div class="flex justify-between items-start z-10">
                        <div>
                            <p class="text-gray-500 text-sm font-medium mb-1">Total Faculty</p>
                            <h3 class="text-3xl font-bold text-gray-900">482</h3>
                        </div>
                        <div class="w-10 h-10 rounded-full bg-secondary-light flex items-center justify-center text-secondary">
                            <i data-fa-i2svg=""><svg class="svg-inline--fa fa-chalkboard-user" aria-hidden="true" focusable="false" data-prefix="fas" data-icon="chalkboard-user" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 512" data-fa-i2svg=""><path fill="currentColor" d="M160 64c0-35.3 28.7-64 64-64H576c35.3 0 64 28.7 64 64V352c0 35.3-28.7 64-64 64H336.8c-11.8-25.5-29.9-47.5-52.4-64H384V320c0-17.7 14.3-32 32-32h64c17.7 0 32 14.3 32 32v32h64V64L224 64v49.1C205.2 102.2 183.3 96 160 96V64zm0 64a96 96 0 1 1 0 192 96 96 0 1 1 0-192zM133.3 352h53.3C260.3 352 320 411.7 320 485.3c0 14.7-11.9 26.7-26.7 26.7H26.7C11.9 512 0 500.1 0 485.3C0 411.7 59.7 352 133.3 352z"></path></svg></i>
                        </div>
                    </div>
                    <div class="z-10 mt-auto">
                        <div class="flex items-center text-xs font-medium text-green-500 bg-green-50 w-fit px-2 py-1 rounded-md">
                            <i class="mr-1" data-fa-i2svg=""><svg class="svg-inline--fa fa-arrow-trend-up" aria-hidden="true" focusable="false" data-prefix="fas" data-icon="arrow-trend-up" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512" data-fa-i2svg=""><path fill="currentColor" d="M384 160c-17.7 0-32-14.3-32-32s14.3-32 32-32H544c17.7 0 32 14.3 32 32V288c0 17.7-14.3 32-32 32s-32-14.3-32-32V205.3L342.6 374.6c-12.5 12.5-32.8 12.5-45.3 0L192 269.3 54.6 406.6c-12.5 12.5-32.8 12.5-45.3 0s-12.5-32.8 0-45.3l160-160c12.5-12.5 32.8-12.5 45.3 0L320 306.7 466.7 160H384z"></path></svg></i>
                            <span>+4% this month</span>
                        </div>
                    </div>
                    <div class="absolute -bottom-4 -right-4 w-24 h-24 bg-green-50 rounded-full opacity-50 group-hover:scale-110 transition-transform"></div>
                </div>

                <div class="bg-white p-5 rounded-2xl shadow-soft flex flex-col justify-between h-40 relative overflow-hidden group hover:shadow-lg transition-shadow">
                    <div class="flex justify-between items-start z-10">
                        <div>
                            <p class="text-gray-500 text-sm font-medium mb-1">Supervisors</p>
                            <h3 class="text-3xl font-bold text-gray-900">64</h3>
                        </div>
                        <div class="w-10 h-10 rounded-full bg-blue-50 flex items-center justify-center text-blue-500">
                            <i data-fa-i2svg=""><svg class="svg-inline--fa fa-user-tie" aria-hidden="true" focusable="false" data-prefix="fas" data-icon="user-tie" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" data-fa-i2svg=""><path fill="currentColor" d="M224 256A128 128 0 1 1 224 0a128 128 0 1 1 0 256zM209.1 359.2l-18.6-31c-6.4-10.7 1.3-24.2 13.7-24.2H224h19.7c12.4 0 20.1 13.6 13.7 24.2l-18.6 31 33.4 123.9 36-146.9c2-8.1 9.8-13.4 17.9-11.3c70.1 17.6 121.9 81 121.9 156.4c0 17-13.8 30.7-30.7 30.7H285.5c-2.1 0-4-.4-5.8-1.1l.3 1.1H168l.3-1.1c-1.8 .7-3.8 1.1-5.8 1.1H30.7C13.8 512 0 498.2 0 481.3c0-75.5 51.9-138.9 121.9-156.4c8.1-2 15.9 3.3 17.9 11.3l36 146.9 33.4-123.9z"></path></svg></i>
                        </div>
                    </div>
                    <div class="z-10 mt-auto">
                        <div class="flex items-center text-xs font-medium text-gray-500 bg-gray-100 w-fit px-2 py-1 rounded-md">
                            <i class="mr-1" data-fa-i2svg=""><svg class="svg-inline--fa fa-minus" aria-hidden="true" focusable="false" data-prefix="fas" data-icon="minus" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" data-fa-i2svg=""><path fill="currentColor" d="M432 256c0 17.7-14.3 32-32 32L48 288c-17.7 0-32-14.3-32-32s14.3-32 32-32l352 0c17.7 0 32 14.3 32 32z"></path></svg></i>
                            <span>No change</span>
                        </div>
                    </div>
                    <div class="absolute -bottom-4 -right-4 w-24 h-24 bg-blue-50 rounded-full opacity-50 group-hover:scale-110 transition-transform"></div>
                </div>

                <div class="bg-gradient-to-br from-primary to-orange-600 p-5 rounded-2xl shadow-lg shadow-orange-200 flex flex-col justify-between h-40 relative overflow-hidden text-white group hover:shadow-xl transition-shadow">
                    <div class="flex justify-between items-start z-10">
                        <div>
                            <p class="text-white/80 text-sm font-medium mb-1">Active Courses</p>
                            <h3 class="text-3xl font-bold">1,208</h3>
                        </div>
                        <div class="w-10 h-10 rounded-full bg-white/20 flex items-center justify-center text-white backdrop-blur-sm">
                            <i data-fa-i2svg=""><svg class="svg-inline--fa fa-book-open" aria-hidden="true" focusable="false" data-prefix="fas" data-icon="book-open" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512" data-fa-i2svg=""><path fill="currentColor" d="M249.6 471.5c10.8 3.8 22.4-4.1 22.4-15.5V78.6c0-4.2-1.6-8.4-5-11C247.4 52 202.4 32 144 32C93.5 32 46.3 45.3 18.1 56.1C6.8 60.5 0 71.7 0 83.8V454.1c0 11.9 12.8 20.2 24.1 16.5C55.6 460.1 105.5 448 144 448c33.9 0 79 14 105.6 23.5zm76.8 0C353 462 398.1 448 432 448c38.5 0 88.4 12.1 119.9 22.6c11.3 3.8 24.1-4.6 24.1-16.5V83.8c0-12.1-6.8-23.3-18.1-27.6C529.7 45.3 482.5 32 432 32c-58.4 0-103.4 20-123 35.6c-3.3 2.6-5 6.8-5 11V456c0 11.4 11.7 19.3 22.4 15.5z"></path></svg></i>
                        </div>
                    </div>
                    <div class="z-10 mt-auto">
                        <button class="text-xs font-bold bg-white text-primary px-3 py-1.5 rounded-lg hover:bg-gray-50 transition-colors">
                            View Analytics
                        </button>
                    </div>
                    <div class="absolute -top-10 -right-10 w-40 h-40 bg-white/10 rounded-full blur-2xl"></div>
                </div>
            </div>
    }
}