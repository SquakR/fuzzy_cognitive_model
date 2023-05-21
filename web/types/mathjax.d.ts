import { MathJaxObject } from 'mathjax-full/ts/components/global'

type MathJax = MathJaxObject & {
  typesetPromise: (elements: HTMLElement[]) => Promise<any>
}

declare global {
  interface Window {
    MathJax: MathJax
  }
}
