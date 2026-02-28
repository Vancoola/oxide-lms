module.exports = {
    content: [
        "./index.html",
        "./src/**/*.rs",
        "../oxide-ui/src/**/*.rs",
        "../oxide-web-common/src/**/*.rs",
    ],
    theme: {
        extend: {
            colors: {
                primary: '#FF8A50', // Extracted peach/orange accent
                'primary-light': '#FFF0E6',
                secondary: '#48C78E', // Extracted green accent
                'secondary-light': '#E6F8EF',
                dark: '#1F2937',
                'gray-bg': '#F5F6FA',
                'card-bg': '#FFFFFF',
                'text-main': '#111827',
                'text-muted': '#6B7280',
            },
            fontFamily: {
                sans: ['Inter', 'sans-serif'],
            },
            borderRadius: {
                'xl': '1rem',
                '2xl': '1.25rem',
                '3xl': '1.5rem',
            },
            boxShadow: {
                'soft': '0 4px 20px -2px rgba(0, 0, 0, 0.05)',
                'card': '0 0 0 1px rgba(0,0,0,0.03), 0 2px 8px rgba(0,0,0,0.04)',
            }
        }
    }
}