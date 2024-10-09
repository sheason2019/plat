import ConfirmInstallPluginModal from "./modals/confirm-install-plugin-modal";
import ConfirmRemovePluginModal from "./modals/confirm-remove-plugin-modal";

export default function ConfirmModal() {
  return (
    <>
      <ConfirmInstallPluginModal />
      <ConfirmRemovePluginModal />
    </>
  );
}
