export {}

declare module 'vuetify/lib/util/colors' {
  interface MainColors {
    base: string
    lighten5: string
    lighten4: string
    lighten3: string
    lighten2: string
    lighten1: string
    darken1: string
    darken2: string
    darken3: string
    darken4: string
    accent1: string
    accent2: string
    accent3: string
    accent4: string
  }
  interface ExtraColors {
    base: string
    lighten5: string
    lighten4: string
    lighten3: string
    lighten2: string
    lighten1: string
    darken1: string
    darken2: string
    darken3: string
    darken4: string
  }
  interface Shades {
    black: string
    white: string
    transparent: string
  }
  interface Colors {
    red: MainColors
    pink: MainColors
    purple: MainColors
    deepPurple: MainColors
    indigo: MainColors
    blue: MainColors
    lightBlue: MainColors
    cyan: MainColors
    teal: MainColors
    green: MainColors
    lightGreen: MainColors
    lime: MainColors
    yellow: MainColors
    amber: MainColors
    orange: MainColors
    deepOrange: MainColors
    brown: ExtraColors
    blueGrey: ExtraColors
    grey: ExtraColors
    shades: Shades
  }

  export default Object as Colors
}
