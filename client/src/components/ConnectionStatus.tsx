import React from 'react';

interface ConnectionStatusProps {
  isConnecting: boolean;
  isConnected: boolean;
  error: string | null;
  onRetry: () => void;
}

const ConnectionStatus: React.FC<ConnectionStatusProps> = ({
  isConnecting,
  isConnected,
  error,
  onRetry
}) => {
  if (isConnecting) {
    return (
      <div className="connection-status">
        <div className="loading-spinner"></div>
        <p>Connecting to game server...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="connection-status error">
        <p>Connection failed: {error}</p>
        <button onClick={onRetry} className="retry-button">
          Retry Connection
        </button>
      </div>
    );
  }

  if (isConnected) {
    return (
      <div className="connection-status success">
        <p>Connected successfully!</p>
      </div>
    );
  }

  return (
    <div className="connection-status">
      <p>Not connected</p>
      <button onClick={onRetry} className="retry-button">
        Connect
      </button>
    </div>
  );
};

export default ConnectionStatus; 