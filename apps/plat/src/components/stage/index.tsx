import EmptyStage from "./empty-stage";
import useStage from "./hooks/use-stage";
import InnerStage from "./inner-stage";
import StandaloneStage from "./standalone-stage";
import UnknownStage from "./unknown-stage";

export default function Stage() {
  const [stage] = useStage();

  if (!stage) return <EmptyStage />;

  switch (stage.entry.target) {
    case "inner":
      return <InnerStage />;
    case "standalone":
      return <StandaloneStage />;
    default:
      return <UnknownStage />;
  }
}
