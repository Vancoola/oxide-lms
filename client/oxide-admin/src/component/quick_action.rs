use leptos::prelude::*;

#[component]
pub fn QuickAction() -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8">
                <div class="lg:col-span-1 bg-white p-6 rounded-2xl shadow-soft">
                    <div class="flex justify-between items-center mb-6">
                        <h2 class="text-lg font-bold text-gray-900">Create New Account</h2>
                        <button class="text-gray-400 hover:text-gray-600"><i data-fa-i2svg=""><svg class="svg-inline--fa fa-ellipsis" aria-hidden="true" focusable="false" data-prefix="fas" data-icon="ellipsis" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" data-fa-i2svg=""><path fill="currentColor" d="M8 256a56 56 0 1 1 112 0A56 56 0 1 1 8 256zm160 0a56 56 0 1 1 112 0 56 56 0 1 1 -112 0zm216-56a56 56 0 1 1 0 112 56 56 0 1 1 0-112z"></path></svg></i></button>
                    </div>

                    <form class="space-y-4">
                        <div>
                            <label class="block text-xs font-medium text-gray-500 mb-1">Account Type</label>
                            <div class="grid grid-cols-3 gap-2">
                                <label class="cursor-pointer">
                                    <input type="radio" name="role" class="peer sr-only" checked=""/>
                                    <div class="flex flex-col items-center justify-center p-3 border border-gray-200 rounded-xl peer-checked:border-primary peer-checked:bg-primary-light peer-checked:text-primary transition-all hover:bg-gray-50">
                                        <i class="mb-1 text-lg" data-fa-i2svg=""><svg class="svg-inline--fa fa-user-graduate" aria-hidden="true" focusable="false" data-prefix="fas" data-icon="user-graduate" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" data-fa-i2svg=""><path fill="currentColor" d="M219.3 .5c3.1-.6 6.3-.6 9.4 0l200 40C439.9 42.7 448 52.6 448 64s-8.1 21.3-19.3 23.5L352 102.9V160c0 70.7-57.3 128-128 128s-128-57.3-128-128V102.9L48 93.3v65.1l15.7 78.4c.9 4.7-.3 9.6-3.3 13.3s-7.6 5.9-12.4 5.9H16c-4.8 0-9.3-2.1-12.4-5.9s-4.3-8.6-3.3-13.3L16 158.4V86.6C6.5 83.3 0 74.3 0 64C0 52.6 8.1 42.7 19.3 40.5l200-40zM111.9 327.7c10.5-3.4 21.8 .4 29.4 8.5l71 75.5c6.3 6.7 17 6.7 23.3 0l71-75.5c7.6-8.1 18.9-11.9 29.4-8.5C401 348.6 448 409.4 448 481.3c0 17-13.8 30.7-30.7 30.7H30.7C13.8 512 0 498.2 0 481.3c0-71.9 47-132.7 111.9-153.6z"></path></svg></i>
                                        <span class="text-[10px] font-semibold">Student</span>
                                    </div>
                                </label>
                                <label class="cursor-pointer">
                                    <input type="radio" name="role" class="peer sr-only"/>
                                    <div class="flex flex-col items-center justify-center p-3 border border-gray-200 rounded-xl peer-checked:border-secondary peer-checked:bg-secondary-light peer-checked:text-secondary transition-all hover:bg-gray-50">
                                        <i class="mb-1 text-lg" data-fa-i2svg=""><svg class="svg-inline--fa fa-chalkboard-user" aria-hidden="true" focusable="false" data-prefix="fas" data-icon="chalkboard-user" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 512" data-fa-i2svg=""><path fill="currentColor" d="M160 64c0-35.3 28.7-64 64-64H576c35.3 0 64 28.7 64 64V352c0 35.3-28.7 64-64 64H336.8c-11.8-25.5-29.9-47.5-52.4-64H384V320c0-17.7 14.3-32 32-32h64c17.7 0 32 14.3 32 32v32h64V64L224 64v49.1C205.2 102.2 183.3 96 160 96V64zm0 64a96 96 0 1 1 0 192 96 96 0 1 1 0-192zM133.3 352h53.3C260.3 352 320 411.7 320 485.3c0 14.7-11.9 26.7-26.7 26.7H26.7C11.9 512 0 500.1 0 485.3C0 411.7 59.7 352 133.3 352z"></path></svg></i>
                                        <span class="text-[10px] font-semibold">Faculty</span>
                                    </div>
                                </label>
                                <label class="cursor-pointer">
                                    <input type="radio" name="role" class="peer sr-only"/>
                                    <div class="flex flex-col items-center justify-center p-3 border border-gray-200 rounded-xl peer-checked:border-blue-500 peer-checked:bg-blue-50 peer-checked:text-blue-500 transition-all hover:bg-gray-50">
                                        <i class="mb-1 text-lg" data-fa-i2svg=""><svg class="svg-inline--fa fa-user-tie" aria-hidden="true" focusable="false" data-prefix="fas" data-icon="user-tie" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" data-fa-i2svg=""><path fill="currentColor" d="M224 256A128 128 0 1 1 224 0a128 128 0 1 1 0 256zM209.1 359.2l-18.6-31c-6.4-10.7 1.3-24.2 13.7-24.2H224h19.7c12.4 0 20.1 13.6 13.7 24.2l-18.6 31 33.4 123.9 36-146.9c2-8.1 9.8-13.4 17.9-11.3c70.1 17.6 121.9 81 121.9 156.4c0 17-13.8 30.7-30.7 30.7H285.5c-2.1 0-4-.4-5.8-1.1l.3 1.1H168l.3-1.1c-1.8 .7-3.8 1.1-5.8 1.1H30.7C13.8 512 0 498.2 0 481.3c0-75.5 51.9-138.9 121.9-156.4c8.1-2 15.9 3.3 17.9 11.3l36 146.9 33.4-123.9z"></path></svg></i>
                                        <span class="text-[10px] font-semibold">Admin</span>
                                    </div>
                                </label>
                            </div>
                        </div>

                        <div class="grid grid-cols-2 gap-4">
                            <div>
                                <label class="block text-xs font-medium text-gray-500 mb-1">First Name</label>
                                <input type="text" class="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50 focus:border-primary"/>
                            </div>
                            <div>
                                <label class="block text-xs font-medium text-gray-500 mb-1">Last Name</label>
                                <input type="text" class="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50 focus:border-primary"/>
                            </div>
                        </div>

                        <div>
                            <label class="block text-xs font-medium text-gray-500 mb-1">Email Address</label>
                            <input type="email" class="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50 focus:border-primary"/>
                        </div>

                        <div>
                            <label class="block text-xs font-medium text-gray-500 mb-1">Password</label>
                            <input type="password" class="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-primary/50 focus:border-primary" placeholder="••••••••"/>
                        </div>

                        <button type="button" class="w-full bg-gray-900 text-white font-semibold py-2.5 rounded-xl hover:bg-gray-800 transition-colors shadow-lg shadow-gray-200">
                            Create Account
                        </button>
                    </form>
                </div>

                <div class="lg:col-span-2 bg-white p-6 rounded-2xl shadow-soft flex flex-col">
                    <div class="flex justify-between items-center mb-4">
                        <div>
                            <h2 class="text-lg font-bold text-gray-900">Registration Trends</h2>
                            <div class="flex items-center gap-4 mt-1">
                                <div class="flex items-center gap-1.5">
                                    <span class="w-2.5 h-2.5 rounded-full bg-primary"></span>
                                    <span class="text-xs text-gray-500">Students</span>
                                </div>
                                <div class="flex items-center gap-1.5">
                                    <span class="w-2.5 h-2.5 rounded-full bg-secondary"></span>
                                    <span class="text-xs text-gray-500">Faculty</span>
                                </div>
                            </div>
                        </div>
                        <select class="bg-gray-50 border border-gray-200 text-gray-600 text-xs rounded-lg focus:ring-primary focus:border-primary block p-2">
                            <option>Last 30 Days</option>
                            <option>Last 6 Months</option>
                            <option>This Year</option>
                        </select>
                    </div>
                </div>
            </div>
    }
}