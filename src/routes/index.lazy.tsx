import { createLazyFileRoute } from "@tanstack/react-router";
import CreateIsolateCard from "../components/isolate/create-isolate-card";
import IsolateCard from "../components/isolate/isolate-card";
import Header from "../components/header";
import useProfile from "../hooks/core/use-profile";

export const Route = createLazyFileRoute("/")({
  component: Index,
});

function Index() {
  const { data: profile } = useProfile();

  return (
    <div>
      <Header title="账号管理" />
      <div className="container mx-auto px-2">
        <div className="grid grid-cols-1 md:grid-cols-2 mt-3 gap-3">
          <CreateIsolateCard />
          {profile?.isolates.map((isolate) => (
            <IsolateCard key={isolate.public_key} isolate={isolate} />
          ))}
        </div>
      </div>
    </div>
  );
}
