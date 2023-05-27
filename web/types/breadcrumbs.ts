import { RouteLocationRaw } from 'vue-router'

export interface BreadcrumbsItem {
  title: string
  to?: RouteLocationRaw
  exact?: boolean
}
