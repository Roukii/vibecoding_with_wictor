import React, { useState, useEffect, useRef } from 'react';
import { Message, Player } from '../module_bindings';

interface ChatProps {
  messages: Message[];
  players: Player[];
  currentPlayer: Player;
  onSendMessage: (text: string) => Promise<void>;
}

const Chat: React.FC<ChatProps> = ({ 
  messages, 
  players, 
  currentPlayer, 
  onSendMessage 
}) => {
  const [inputText, setInputText] = useState('');
  const [isSending, setIsSending] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!inputText.trim() || isSending) return;

    const messageText = inputText.trim();
    setInputText('');
    setIsSending(true);

    try {
      await onSendMessage(messageText);
    } finally {
      setIsSending(false);
    }
  };

  const getPlayerName = (identity: any) => {
    const player = players.find(p => p.identity.isEqual(identity));
    return player?.name || 'Unknown Player';
  };

  const formatTime = (timestamp: any) => {
    const date = new Date(timestamp.millis);
    return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  };

  // Auto-scroll to bottom when new messages arrive
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  return (
    <div className="chat">
      <div className="chat-header">
        <h3>Chat</h3>
        <span className="player-count">{players.length} players online</span>
      </div>
      
      <div className="chat-messages">
        {messages.map((message) => {
          const isCurrentPlayer = message.sender.isEqual(currentPlayer.identity);
          return (
            <div
              key={message.id.toString()}
              className={`message ${isCurrentPlayer ? 'own-message' : ''}`}
            >
              <div className="message-header">
                <span className="sender-name">
                  {getPlayerName(message.sender)}
                </span>
                <span className="message-time">
                  {formatTime(message.sent)}
                </span>
              </div>
              <div className="message-text">{message.text}</div>
            </div>
          );
        })}
        <div ref={messagesEndRef} />
      </div>

      <form onSubmit={handleSubmit} className="chat-input">
        <input
          type="text"
          value={inputText}
          onChange={(e) => setInputText(e.target.value)}
          placeholder="Type a message..."
          className="message-input"
          maxLength={500}
          disabled={isSending}
        />
        <button
          type="submit"
          className="send-button"
          disabled={!inputText.trim() || isSending}
        >
          Send
        </button>
      </form>
    </div>
  );
};

export default Chat; 