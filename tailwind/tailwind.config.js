/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["../templates/**/*.html"],
  theme: {
    extend: {
      colors: {
        'pastel-yellow': '#FAEDCB',
        'pastel-seafoam': '#C9E4DE',
        'pastel-blue': '#C6DEF1',
        'pastel-purple': '#DBCDF0',
        'pastel-pink': '#F2C6DE',
        'pastel-salmon': '#F7D9C4',
      }
    },
    listStyleType: {
      none: 'none',
      disc: 'disc',
      decimal: 'decimal',
      square: 'square',
      roman: 'upper-roman',
    },
  },
  plugins: [],
}
