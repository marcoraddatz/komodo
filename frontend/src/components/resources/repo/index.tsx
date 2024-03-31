import { TagsWithBadge } from "@components/tags";
import { useRead } from "@lib/hooks";
import { RequiredResourceComponents } from "@types";
import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { DataTable } from "@ui/data-table";
import { GitBranch } from "lucide-react";
import { Link } from "react-router-dom";
import { RepoConfig } from "./config";

const useRepo = (id?: string) =>
  useRead("ListRepos", {}).data?.find((d) => d.id === id);

export const RepoComponents: RequiredResourceComponents = {
  Name: ({ id }: { id: string }) => <>{useRepo(id)?.name}</>,
  Description: ({ id }) => <>{id}</>,
  Icon: () => <GitBranch className="w-4 h-4" />,
  Info: [],
  Status: () => <></>,
  Actions: () => <></>,
  New: () => <></>,
  Page: {
    Config: RepoConfig,
  },
  Table: () => {
    const alerters = useRead("ListAlerters", {}).data;
    return (
      <DataTable
        data={alerters ?? []}
        columns={[
          {
            accessorKey: "id",
            header: "Name",
            cell: ({ row }) => {
              const id = row.original.id;
              return (
                <Link to={`/repos/${id}`} className="flex items-center gap-2">
                  <RepoComponents.Icon id={id} />
                  <RepoComponents.Name id={id} />
                </Link>
              );
            },
          },
          {
            header: "Tags",
            cell: ({ row }) => {
              return (
                <div className="flex gap-1">
                  <TagsWithBadge tag_ids={row.original.tags} />
                </div>
              );
            },
          },
        ]}
      />
    );
  },
  Dashboard: () => {
    const repo_count = useRead("ListRepos", {}).data?.length;
    return (
      <Link to="/repos/" className="w-full">
        <Card>
          <CardHeader className="justify-between">
            <div>
              <CardTitle>Repos</CardTitle>
              <CardDescription>{repo_count} Total</CardDescription>
            </div>
            <GitBranch className="w-4 h-4" />
          </CardHeader>
        </Card>
      </Link>
    );
  },
};
