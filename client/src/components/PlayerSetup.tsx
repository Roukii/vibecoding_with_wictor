import React, { useState } from 'react';

interface PlayerSetupProps {
  onSetName: (name: string) => Promise<void>;
}

const PlayerSetup: React.FC<PlayerSetupProps> = ({ onSetName }) => {
  const [name, setName] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim() || isSubmitting) return;

    setIsSubmitting(true);
    try {
      await onSetName(name.trim());
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="player-setup">
      <div className="setup-container">
        <h2>Welcome to the Game!</h2>
        <p>Enter your name to start playing</p>
        
        <form onSubmit={handleSubmit} className="name-form">
          <input
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            placeholder="Enter your name"
            className="name-input"
            minLength={2}
            maxLength={20}
            required
            disabled={isSubmitting}
          />
          <button 
            type="submit" 
            className="start-button"
            disabled={!name.trim() || isSubmitting}
          >
            {isSubmitting ? 'Starting...' : 'Start Playing'}
          </button>
        </form>
      </div>
    </div>
  );
};

export default PlayerSetup; 