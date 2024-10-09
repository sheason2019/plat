import ConfirmInstallPluginModal from "./modals/confirm-install-plugin-modal";
import ConfirmDeletePluginModal from "./modals/confirm-delete-plugin-modal";

export default function ConfirmModal() {
  return (
    <>
      <ConfirmInstallPluginModal />
      <ConfirmDeletePluginModal />
    </>
  );
}
