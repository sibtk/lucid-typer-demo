import { useToastStore } from "../../stores/toast-store";

export function ToastContainer() {
  const toasts = useToastStore((s) => s.toasts);

  if (toasts.length === 0) return null;

  return (
    <div className="fixed bottom-6 left-0 right-0 z-50 flex flex-col items-center gap-2 pointer-events-none">
      {toasts.map((toast) => (
        <div
          key={toast.id}
          className={`pointer-events-auto rounded-lg px-4 py-2 text-[13px] font-medium flex items-center gap-2.5 bg-surface-raised border border-white/[0.08] shadow-lg ${
            toast.type === "success"
              ? "border-l-2 border-l-green"
              : toast.type === "warning"
              ? "border-l-2 border-l-amber"
              : "border-l-2 border-l-white/20"
          } ${
            toast.dismissing ? "animate-toast-out" : "animate-toast-in"
          }`}
        >
          <div
            className={`w-1.5 h-1.5 rounded-full shrink-0 ${
              toast.type === "success"
                ? "bg-green"
                : toast.type === "warning"
                ? "bg-amber"
                : "bg-text-muted"
            }`}
            style={
              toast.type === "success"
                ? { boxShadow: "0 0 6px rgba(52, 211, 153, 0.5)" }
                : undefined
            }
          />
          <span className="text-text-secondary whitespace-nowrap">{toast.message}</span>
        </div>
      ))}
    </div>
  );
}
