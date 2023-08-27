import type { Config } from 'tailwindcss'

const config: Config = {
  content: [
    './pages/**/*.{js,ts,jsx,tsx,mdx}',
    './components/**/*.{js,ts,jsx,tsx,mdx}',
    './app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {
      backgroundImage: {
        'gradient-radial': 'radial-gradient(var(--tw-gradient-stops))',
        'gradient-conic':
          'conic-gradient(from 180deg at 50% 50%, var(--tw-gradient-stops))',
        'image-ship-aircraftcarrier': 'url(/images/ship-aircraftcarrier.png)',
        'image-ship-battleship': 'url(/images/ship-battleship.png)',
        'image-ship-cruiser': 'url(/images/ship-cruiser.png)',
        'image-ship-frigate': 'url(/images/ship-frigate.png)',
        'image-ship-submarine': 'url(/images/ship-submarine.png)',
      },
      backgroundSize: {
        'image-ship': 'auto var(--bg-ship-size)',
      }
    },
    fontSize: {
      xxs: '0.5rem',
      xs: '0.75rem',
      sm: '0.875rem',
      base: '1rem',
      lg: '1.125rem',
      xl: '1.25rem',
      '2xl': '1.5rem',
      '3xl': '1.875rem',
      '4xl': '2.25rem',
      '5xl': '3rem',
      '6xl': '3.75rem',
      'icon': 'var(--position-icon-size)',
      'button': 'var(--button-font-size)',
    }
  },
  plugins: [],
}
export default config
