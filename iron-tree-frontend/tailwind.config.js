/** @type {import('tailwindcss').Config} */
export default {
    darkMode: "class",
    content: [
        "./index.html",
        "./src/**/*.{js,ts,jsx,tsx}",
    ],
    theme: {
        extend: {},
    },
    corePlugins: {
        preflight: false    // 关闭样式覆盖
    },
    plugins: [],
}
