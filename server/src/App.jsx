import { Routes, Route } from "react-router-dom";
import { Layout } from "./Layout";
import { Home } from "./pages/Home";
import { Server } from "./Server";
import { Clients } from "./pages/ClientList";
import { ClientView } from "./pages/ClientView";
import { FileManager } from "./pages/FileManager";
import { RemoteShell } from "./pages/RemoteShell";
import { Settings } from "./pages/Settings";
import { RATProvider } from "./rat/RATProvider";
import toast, { Toaster } from "react-hot-toast";

export const App = () => {
  return (
    <RATProvider>
      <Routes>
        <Route path="/" element={<Server />} />

        <Route path="/" element={<Layout />}>
          <Route path="/home" element={<Home />} />
          <Route path="/clients" element={<Clients />} />
          <Route path="/clients/:id" element={<ClientView />} />
          <Route path="/clients/:id/files" element={<FileManager />} />
          <Route path="/clients/:id/shell" element={<RemoteShell />} />
          <Route path="/settings" element={<Settings />} />
        </Route>
      </Routes>
      <Toaster position="bottom-right" reverseOrder={false} />
    </RATProvider>
  );
};
