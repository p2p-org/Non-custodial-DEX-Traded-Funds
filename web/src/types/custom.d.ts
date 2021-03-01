export interface SvgIconType {
  viewBox: string;
  id: string;
}

declare module '*-icon.svg' {
  const content: SvgIconType;
  export default content;
}
