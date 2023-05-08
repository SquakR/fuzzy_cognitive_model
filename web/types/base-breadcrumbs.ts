import { RouteLocationRaw } from 'vue-router'

export interface BreadcrumbItem {
  href?: string | undefined
  replace?: boolean | undefined
  to?: RouteLocationRaw | undefined
  exact?: boolean | undefined
  title: string
  disabled?: boolean
}
