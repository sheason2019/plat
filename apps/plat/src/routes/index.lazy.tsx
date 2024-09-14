import { createLazyFileRoute } from "@tanstack/react-router";
import CreateIsolateCard from "../components/isolate/create-isolate-card";
import IsolateCard from "../components/isolate/isolate-card";
import useProfile from "../hooks/core/use-profile";
import Header from "../components/header";

export const Route = createLazyFileRoute("/")({
  component: Index,
});

function Index() {
  const { data: profile } = useProfile();

  return (
    <div className="container mx-auto px-2">
      <Header title="账号管理" />
      <div className="grid grid-cols-1 md:grid-cols-2 mt-3 gap-3">
        <CreateIsolateCard />
        {profile?.daemons.map((isolate) => (
          <IsolateCard key={isolate.public_key} isolate={isolate} />
        ))}
      </div>
    </div>
  );
}
