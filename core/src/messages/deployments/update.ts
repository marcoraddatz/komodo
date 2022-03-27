import { Deployment, User } from "@monitor/types";
import { deploymentChangelog, UPDATE_DEPLOYMENT } from "@monitor/util";
import { FastifyInstance } from "fastify";
import { PERMISSIONS_DENY_LOG } from "../../config";
import { clonePeriphery } from "../../util/periphery/git";
import { addDeploymentUpdate } from "../../util/updates";
import cloneRepo from "./clone";

async function updateDeployment(
  app: FastifyInstance,
  user: User,
  { deployment, note }: { deployment: Deployment; note?: string }
) {
  const preDeployment = await app.deployments.findById(deployment._id!);
  if (!preDeployment) return;
  if (user.permissions! < 2 && !preDeployment.owners.includes(user.username)) {
    addDeploymentUpdate(
      app,
      deployment._id!,
      UPDATE_DEPLOYMENT,
      "Update Deployment (DENIED)",
      PERMISSIONS_DENY_LOG,
      user.username,
      note,
      true
    );
    return;
  }
  try {
    // this assumes no change to deployment name (ie cannot rename deployments after created)
    if (
      deployment.repo !== preDeployment.repo ||
      deployment.branch !== preDeployment.branch
    ) {
      const server =
        deployment.serverID === app.core._id
          ? undefined
          : await app.servers.findById(deployment.serverID!);
      if (server) {
        await clonePeriphery(server, deployment);
      } else {
        await cloneRepo(app, user, deployment);
      }
    }
    // make sure owners cant be updated this way
    (deployment.owners as any) = false;
    await app.deployments.updateById(deployment._id!, deployment);
    addDeploymentUpdate(
      app,
      deployment._id!,
      UPDATE_DEPLOYMENT,
      "Update Deployment",
      {
        stdout: deploymentChangelog(preDeployment, deployment),
      },
      user.username,
      note
    );
  } catch (error) {
    addDeploymentUpdate(
      app,
      deployment._id!,
      UPDATE_DEPLOYMENT,
      "Update Deployment (ERROR)",
      {
        stderr: JSON.stringify(error),
      },
      user.username,
      note,
      true
    );
  }
}

export default updateDeployment;
