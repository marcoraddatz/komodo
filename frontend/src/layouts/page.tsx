import { ReactNode } from "react";

interface PageProps {
  title: ReactNode;
  subtitle: ReactNode;
  actions: ReactNode;
  content: ReactNode;
}

export const Page = ({ title, subtitle, actions, content }: PageProps) => (
  <div className="flex flex-col gap-12">
    <div className="flex justify-between">
      <div className="flex flex-col">
        {title}
        {subtitle}
      </div>
      {actions}
    </div>
    {content}
  </div>
);
