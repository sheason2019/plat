import clsx from "clsx";

export default function EmptyStage() {
  return (
    <div
      className={clsx(
        "flex-1 flex justify-center items-center",
        "text-6xl font-mono font-bold text-default-300",
        "select-none"
      )}
    >
      Plat
    </div>
  );
}
