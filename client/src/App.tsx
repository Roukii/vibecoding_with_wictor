import React, { useEffect, useState } from 'react';
import './App.css';
import {
  DbConnection,
  type ErrorContext,
  type EventContext,
  Message,
  User,
} from './module_bindings';
import { Identity, Timestamp } from '@clockworklabs/spacetimedb-sdk';

export type PrettyMessage = {
  id: bigint;
  sender: Identity;
  senderName: string;
  senderAvatar: string | null;
  text: string;
  sent: Timestamp;
};

function useMessages(conn: DbConnection | null): Message[] {
  const [messages, setMessages] = useState<Message[]>([]);

  useEffect(() => {
    if (!conn) return;
    const onInsert = (_ctx: EventContext, message: Message) => {
      setMessages(prev => [...prev, message]);
    };
    conn.db.message.onInsert(onInsert);

    const onDelete = (_ctx: EventContext, message: Message) => {
      setMessages(prev =>
        prev.filter(m => m.id !== message.id)
      );
    };
    conn.db.message.onDelete(onDelete);

    return () => {
      conn.db.message.removeOnInsert(onInsert);
      conn.db.message.removeOnDelete(onDelete);
    };
  }, [conn]);

  return messages;
}

function useUsers(conn: DbConnection | null): Map<string, User> {
  const [users, setUsers] = useState<Map<string, User>>(new Map());

  useEffect(() => {
    if (!conn) return;
    const onInsert = (_ctx: EventContext, user: User) => {
      setUsers(prev => new Map(prev.set(user.identity.toHexString(), user)));
    };
    conn.db.user.onInsert(onInsert);

    const onUpdate = (_ctx: EventContext, oldUser: User, newUser: User) => {
      setUsers(prev => {
        prev.delete(oldUser.identity.toHexString());
        return new Map(prev.set(newUser.identity.toHexString(), newUser));
      });
    };
    conn.db.user.onUpdate(onUpdate);

    const onDelete = (_ctx: EventContext, user: User) => {
      setUsers(prev => {
        prev.delete(user.identity.toHexString());
        return new Map(prev);
      });
    };
    conn.db.user.onDelete(onDelete);

    return () => {
      conn.db.user.removeOnInsert(onInsert);
      conn.db.user.removeOnUpdate(onUpdate);
      conn.db.user.removeOnDelete(onDelete);
    };
  }, [conn]);

  return users;
}

// Generate a deterministic avatar based on identity
function generateAvatarUrl(identity: string): string {
  // Using DiceBear API for generated avatars based on seed
  return `https://api.dicebear.com/7.x/avataaars/svg?seed=${identity}`;
}

function UserAvatar({ user, size = 32 }: { user: User | undefined, size?: number }) {
  const avatarUrl = user?.avatarUrl || generateAvatarUrl(user?.identity.toHexString() || 'default');
  const userName = user?.name || user?.identity.toHexString().substring(0, 8) || 'Unknown';

  return (
    <img
      src={avatarUrl}
      alt={`${userName}'s avatar`}
      style={{
        width: size,
        height: size,
        borderRadius: '50%',
        objectFit: 'cover',
        marginRight: '8px',
        border: '2px solid #ddd'
      }}
      onError={(e) => {
        // Fallback to generated avatar if custom avatar fails to load
        const target = e.target as HTMLImageElement;
        target.src = generateAvatarUrl(user?.identity.toHexString() || 'default');
      }}
    />
  );
}

function App() {
  const [newName, setNewName] = useState('');
  const [newAvatar, setNewAvatar] = useState('');
  const [settingName, setSettingName] = useState(false);
  const [settingAvatar, setSettingAvatar] = useState(false);
  const [systemMessage, setSystemMessage] = useState('');
  const [newMessage, setNewMessage] = useState('');
  const [connected, setConnected] = useState<boolean>(false);
  const [identity, setIdentity] = useState<Identity | null>(null);
  const [conn, setConn] = useState<DbConnection | null>(null);
  const [deletingMessages, setDeletingMessages] = useState<Set<bigint>>(new Set());

  useEffect(() => {
    const subscribeToQueries = (conn: DbConnection, queries: string[]) => {
      conn
        ?.subscriptionBuilder()
        .onApplied(() => {
          console.log('SDK client cache initialized.');
        })
        .subscribe(queries);
    };

    const onConnect = (
      conn: DbConnection,
      identity: Identity,
      token: string
    ) => {
      setIdentity(identity);
      setConnected(true);
      localStorage.setItem('auth_token', token);
      console.log(
        'Connected to SpacetimeDB with identity:',
        identity.toHexString()
      );
      conn.reducers.onSendMessage(() => {
        console.log('Message sent.');
      });

      conn.reducers.onDeleteMessage(() => {
        console.log('Message deleted.');
      });

      conn.reducers.onSetAvatar(() => {
        console.log('Avatar updated.');
      });

      subscribeToQueries(conn, ['SELECT * FROM message', 'SELECT * FROM user']);
    };

    const onDisconnect = () => {
      console.log('Disconnected from SpacetimeDB');
      setConnected(false);
    };

    const onConnectError = (_ctx: ErrorContext, err: Error) => {
      console.log('Error connecting to SpacetimeDB:', err);
    };

    setConn(
      DbConnection.builder()
        .withUri('ws://localhost:3000')
        .withModuleName('quickstart-chat')
        .withToken(localStorage.getItem('auth_token') || '')
        .onConnect(onConnect)
        .onDisconnect(onDisconnect)
        .onConnectError(onConnectError)
        .build()
    );
  }, []);

  useEffect(() => {
    if (!conn) return;
    conn.db.user.onInsert((_ctx, user) => {
      if (user.online) {
        const name = user.name || user.identity.toHexString().substring(0, 8);
        setSystemMessage(prev => prev + `\n${name} has connected.`);
      }
    });
    conn.db.user.onUpdate((_ctx, oldUser, newUser) => {
      const name =
        newUser.name || newUser.identity.toHexString().substring(0, 8);
      if (oldUser.online === false && newUser.online === true) {
        setSystemMessage(prev => prev + `\n${name} has connected.`);
      } else if (oldUser.online === true && newUser.online === false) {
        setSystemMessage(prev => prev + `\n${name} has disconnected.`);
      }
    });
  }, [conn]);

  const messages = useMessages(conn);
  const users = useUsers(conn);

  const prettyMessages: PrettyMessage[] = messages
    .sort((a, b) => (a.sent > b.sent ? 1 : -1))
    .map(message => {
      const user = users.get(message.sender.toHexString());
      return {
        id: message.id,
        sender: message.sender,
        senderName: user?.name || message.sender.toHexString().substring(0, 8),
        senderAvatar: user?.avatarUrl || null,
        text: message.text,
        sent: message.sent,
      };
    });

  if (!conn || !connected || !identity) {
    return (
      <div className="App">
        <h1>Connecting...</h1>
      </div>
    );
  }

  const currentUser = users.get(identity?.toHexString());
  const name = currentUser?.name || identity?.toHexString().substring(0, 8) || '';

  const onSubmitNewName = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setSettingName(false);
    conn.reducers.setName(newName);
  };

  const onSubmitNewAvatar = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setSettingAvatar(false);
    conn.reducers.setAvatar(newAvatar);
  };

  const onMessageSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setNewMessage('');
    conn.reducers.sendMessage(newMessage);
  };

  const onDeleteMessage = async (messageId: bigint) => {
    if (deletingMessages.has(messageId)) return;
    
    setDeletingMessages(prev => new Set(prev).add(messageId));
    
    try {
      await conn.reducers.deleteMessage(messageId);
    } catch (error) {
      console.error('Failed to delete message:', error);
      // Show error to user if needed
    } finally {
      setDeletingMessages(prev => {
        const newSet = new Set(prev);
        newSet.delete(messageId);
        return newSet;
      });
    }
  };

  const isOwnMessage = (message: PrettyMessage): boolean => {
    return identity?.toHexString() === message.sender.toHexString();
  };

  return (
    <div className="App">
      <div className="profile">
        <h1>Profile</h1>
        <div style={{ display: 'flex', alignItems: 'center', marginBottom: '20px' }}>
          <UserAvatar user={currentUser} size={64} />
          <div>
            <h3 style={{ margin: '0 0 5px 0' }}>{name}</h3>
            <p style={{ margin: 0, fontSize: '0.8em', color: '#666' }}>
              {identity?.toHexString().substring(0, 16)}...
            </p>
          </div>
        </div>
        
        {!settingName ? (
          <div style={{ marginBottom: '10px' }}>
            <button
              onClick={() => {
                setSettingName(true);
                setNewName(name);
              }}
            >
              Edit Name
            </button>
          </div>
        ) : (
          <form onSubmit={onSubmitNewName} style={{ marginBottom: '10px' }}>
            <input
              type="text"
              aria-label="name input"
              value={newName}
              onChange={e => setNewName(e.target.value)}
              placeholder="Enter your name"
            />
            <button type="submit">Save Name</button>
            <button type="button" onClick={() => setSettingName(false)}>Cancel</button>
          </form>
        )}

        {!settingAvatar ? (
          <div>
            <button
              onClick={() => {
                setSettingAvatar(true);
                setNewAvatar(currentUser?.avatarUrl || '');
              }}
            >
              Edit Avatar
            </button>
          </div>
        ) : (
          <form onSubmit={onSubmitNewAvatar}>
            <input
              type="url"
              aria-label="avatar URL input"
              value={newAvatar}
              onChange={e => setNewAvatar(e.target.value)}
              placeholder="Enter avatar URL (https://...)"
            />
            <button type="submit">Save Avatar</button>
            <button type="button" onClick={() => setSettingAvatar(false)}>Cancel</button>
          </form>
        )}
      </div>

      <div className="message">
        <h1>Messages</h1>
        {prettyMessages.length < 1 && <p>No messages</p>}
        <div>
          {prettyMessages.map((message) => {
            const messageUser = users.get(message.sender.toHexString());
            return (
              <div key={message.id} style={{ 
                border: '1px solid #ccc', 
                margin: '10px 0', 
                padding: '10px',
                borderRadius: '5px',
                position: 'relative'
              }}>
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
                  <div style={{ flex: 1, display: 'flex', alignItems: 'flex-start' }}>
                    <UserAvatar user={messageUser} size={40} />
                    <div style={{ flex: 1 }}>
                      <p style={{ margin: '0 0 5px 0' }}>
                        <b>{message.senderName}</b>
                        <span style={{ fontSize: '0.8em', color: '#666', marginLeft: '10px' }}>
                          {message.sent.toDate().toLocaleDateString()}
                        </span>
                      </p>
                      <p style={{ margin: '0' }}>{message.text}</p>
                    </div>
                  </div>
                  {isOwnMessage(message) && (
                    <button
                      onClick={() => onDeleteMessage(message.id)}
                      disabled={deletingMessages.has(message.id)}
                      style={{
                        backgroundColor: '#ff4444',
                        color: 'white',
                        border: 'none',
                        padding: '5px 10px',
                        borderRadius: '3px',
                        cursor: deletingMessages.has(message.id) ? 'not-allowed' : 'pointer',
                        fontSize: '0.8em',
                        marginLeft: '10px'
                      }}
                    >
                      {deletingMessages.has(message.id) ? 'Deleting...' : 'Delete'}
                    </button>
                  )}
                </div>
              </div>
            );
          })}
        </div>
      </div>

      <div className="system" style={{ whiteSpace: 'pre-wrap' }}>
        <h1>System</h1>
        <div>
          <p>{systemMessage}</p>
        </div>
      </div>

      <div className="new-message">
        <form
          onSubmit={onMessageSubmit}
          style={{
            display: 'flex',
            flexDirection: 'column',
            width: '50%',
            margin: '0 auto',
          }}
        >
          <h3>New Message</h3>
          <textarea
            aria-label="message input"
            value={newMessage}
            onChange={e => setNewMessage(e.target.value)}
          ></textarea>
          <button type="submit">Send</button>
        </form>
      </div>
    </div>
  );
}

export default App;