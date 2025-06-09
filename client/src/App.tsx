import { useEffect, useState, useCallback } from 'react';
import './App.css';
import { DbConnection, Player, Map, Message, Entity, User } from './module_bindings';

interface GameState {
  connection: DbConnection | null;
  isConnected: boolean;
  currentUser: User | null;
  currentPlayer: Player | null;
  users: User[];
  players: Player[];
  maps: Map[];
  currentMap: Map | null;
  messages: Message[];
  entities: Entity[];
}

function App() {
  const [gameState, setGameState] = useState<GameState>({
    connection: null,
    isConnected: false,
    currentUser: null,
    currentPlayer: null,
    users: [],
    players: [],
    maps: [],
    currentMap: null,
    messages: [],
    entities: []
  });

  const [isConnecting, setIsConnecting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [playerName, setPlayerName] = useState('');
  const [newMessage, setNewMessage] = useState('');

  const connectToGame = useCallback(async () => {
    if (isConnecting || gameState.isConnected) return;
    
    setIsConnecting(true);
    setError(null);

    try {
      const connection = DbConnection.builder()
        .withUri("ws://127.0.0.1:3000")
        .withModuleName("quickstart-chat")
        .onConnect((conn, identity) => {
          setGameState(prev => ({ ...prev, isConnected: true }));

          // Set up subscriptions AFTER connection is established
          conn.subscriptionBuilder()
            .onApplied(() => {
              // Subscription applied - client cache initialized
            })
            .onError((ctx) => {
              console.error('Subscription error:', ctx.event);
            })
            .subscribe([
              'SELECT * FROM user',
              'SELECT * FROM player',
              'SELECT * FROM map', 
              'SELECT * FROM message',
              'SELECT * FROM entity'
            ]);

          // Listen for table updates AFTER connection is established
          conn.db.user.onInsert((_, user) => {
            setGameState(prev => {
              const existingUserIndex = prev.users.findIndex(u => 
                u.identity.isEqual(user.identity)
              );
              
              let newUsers;
              if (existingUserIndex >= 0) {
                newUsers = [...prev.users];
                newUsers[existingUserIndex] = user;
              } else {
                newUsers = [...prev.users, user];
              }

              // Check if this is the current user
              let currentUser = prev.currentUser;
              if (conn.identity && user.identity.isEqual(conn.identity)) {
                currentUser = user;
              }

              return {
                ...prev,
                users: newUsers,
                currentUser
              };
            });
          });

          conn.db.user.onUpdate((_, _oldUser, newUser) => {
            setGameState(prev => {
              const newUsers = prev.users.map(u => 
                u.identity.isEqual(newUser.identity) ? newUser : u
              );

              let currentUser = prev.currentUser;
              if (conn.identity && newUser.identity.isEqual(conn.identity)) {
                currentUser = newUser;
              }

              return {
                ...prev,
                users: newUsers,
                currentUser
              };
            });
          });

          conn.db.player.onInsert((_, player) => {
            setGameState(prev => {
              const existingPlayerIndex = prev.players.findIndex(p => 
                p.identity.isEqual(player.identity)
              );
              
              let newPlayers;
              if (existingPlayerIndex >= 0) {
                newPlayers = [...prev.players];
                newPlayers[existingPlayerIndex] = player;
              } else {
                newPlayers = [...prev.players, player];
              }

              // Check if this is the current player
              let currentPlayer = prev.currentPlayer;
              if (conn.identity && player.identity.isEqual(conn.identity)) {
                currentPlayer = player;
              }

              return {
                ...prev,
                players: newPlayers,
                currentPlayer
              };
            });
          });

          conn.db.player.onUpdate((_, _oldPlayer, newPlayer) => {
            setGameState(prev => {
              const newPlayers = prev.players.map(p => 
                p.identity.isEqual(newPlayer.identity) ? newPlayer : p
              );

              let currentPlayer = prev.currentPlayer;
              if (conn.identity && newPlayer.identity.isEqual(conn.identity)) {
                currentPlayer = newPlayer;
              }

              return {
                ...prev,
                players: newPlayers,
                currentPlayer
              };
            });
          });

          conn.db.map.onInsert((_, map) => {
            setGameState(prev => {
              const newMaps = [...prev.maps, map];
              let currentMap = prev.currentMap;
              
              // If no current map and this is the starting town, set it as current
              if (!currentMap && map.isStartingTown) {
                currentMap = map;
              }
              
              return {
                ...prev,
                maps: newMaps,
                currentMap
              };
            });
          });

          conn.db.message.onInsert((_, message) => {
            setGameState(prev => {
              // Check if message already exists to prevent duplicates
              const existingMessage = prev.messages.find(m => 
                m.id === message.id
              );
              
              if (existingMessage) {
                return prev; // Don't add duplicate
              }
              
              return {
                ...prev,
                messages: [...prev.messages, message].slice(-100) // Keep last 100 messages
              };
            });
          });

          conn.db.message.onDelete((_, message) => {
            setGameState(prev => ({
              ...prev,
              messages: prev.messages.filter(m => m.id !== message.id)
            }));
          });

          conn.db.entity.onInsert((_, entity) => {
            setGameState(prev => ({
              ...prev,
              entities: [...prev.entities, entity]
            }));
          });

          conn.db.entity.onUpdate((_, _oldEntity, newEntity) => {
            setGameState(prev => ({
              ...prev,
              entities: prev.entities.map(e => e.id === newEntity.id ? newEntity : e)
            }));
          });

          conn.db.entity.onDelete((_, entity) => {
            setGameState(prev => ({
              ...prev,
              entities: prev.entities.filter(e => e.id !== entity.id)
            }));
          });
        })
        .onDisconnect(() => {
          console.log('Disconnected from SpaceTimeDB');
          setGameState({
            connection: null,
            isConnected: false,
            currentUser: null,
            currentPlayer: null,
            users: [],
            players: [],
            maps: [],
            currentMap: null,
            messages: [],
            entities: []
          });
        })
        .onConnectError((ctx) => {
          console.error('Connection error:', ctx.event);
          setError(ctx.event?.message || 'Connection failed');
        })
        .build();

      setGameState(prev => ({ ...prev, connection }));

      // Create player entity after connection is established
      // We'll do this in a separate effect that watches for connection state

    } catch (err) {
      console.error('Failed to connect:', err);
      setError(err instanceof Error ? err.message : 'Connection failed');
    } finally {
      setIsConnecting(false);
    }
  }, [isConnecting, gameState.isConnected]);

  const setPlayerNameHandler = useCallback(async (name: string) => {
    if (!gameState.connection || !gameState.isConnected) {
      setError('Connection not ready. Please wait and try again.');
      return;
    }
    
    try {
      await gameState.connection.reducers.setName(name);
    } catch (err) {
      console.error('Failed to set name:', err);
      if (err instanceof Error && err.message.includes('CONNECTING')) {
        setError('Connection not ready. Please wait and try again.');
      } else {
        setError(err instanceof Error ? err.message : 'Failed to set name');
      }
    }
  }, [gameState.connection, gameState.isConnected, gameState.currentUser]);

  const sendMessage = useCallback(async (text: string) => {
    if (!gameState.connection || !gameState.isConnected) return;
    
    try {
      await gameState.connection.reducers.sendMessage(text);
    } catch (err) {
      console.error('Failed to send message:', err);
      if (err instanceof Error && err.message.includes('CONNECTING')) {
        console.warn('Cannot send message: Connection not ready');
      }
    }
  }, [gameState.connection, gameState.isConnected]);

  const movePlayer = useCallback(async (x: number, y: number) => {
    if (!gameState.connection || !gameState.isConnected) return;
    
    try {
      await gameState.connection.reducers.movePlayer(x, y);
    } catch (err) {
      console.error('Failed to move player:', err);
      if (err instanceof Error && err.message.includes('CONNECTING')) {
        console.warn('Cannot move player: Connection not ready');
      }
    }
  }, [gameState.connection, gameState.isConnected]);

  // Auto-connect on component mount
  useEffect(() => {
    connectToGame();
  }, [connectToGame]);

  // Create player entity after connection is established
  useEffect(() => {
    if (gameState.isConnected && gameState.connection && !gameState.currentPlayer) {
      const createPlayerEntity = async () => {
        try {
          await gameState.connection!.reducers.createPlayerEntity();
        } catch (err) {
          console.warn('Failed to create player entity:', err);
        }
      };
      
      // Add a small delay to ensure connection is fully stable
      const timer = setTimeout(createPlayerEntity, 500);
      return () => clearTimeout(timer);
    }
  }, [gameState.isConnected, gameState.connection, gameState.currentPlayer]);



  if (!gameState.isConnected) {
    return (
      <div className="app">
        <div className="connection-screen">
          <div className="connection-container">
            <h1>Multiplayer Game</h1>
            <div className="connection-status">
              {isConnecting && (
                <>
                  <div className="loading-spinner"></div>
                  <p>Connecting to game server...</p>
                </>
              )}
              
              {error && (
                <div className="connection-status error">
                  <p>Connection failed: {error}</p>
                  <button onClick={connectToGame} className="retry-button">
                    Retry Connection
                  </button>
                </div>
              )}
              
              {!isConnecting && !error && (
                <div className="connection-status">
                  <p>Not connected</p>
                  <button onClick={connectToGame} className="retry-button">
                    Connect
                  </button>
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    );
  }

  if (!gameState.currentUser || !gameState.currentUser.name) {
    return (
      <div className="app">
        <div className="setup-screen">
          <div className="player-setup">
            <div className="setup-container">
              <h2>Welcome to the Game!</h2>
              <p>Enter your name to start playing</p>
              
              <form 
                onSubmit={(e) => {
                  e.preventDefault();
                  if (playerName.trim()) {
                    setPlayerNameHandler(playerName.trim());
                  }
                }} 
                className="name-form"
              >
                <input
                  type="text"
                  value={playerName}
                  onChange={(e) => setPlayerName(e.target.value)}
                  placeholder="Enter your name"
                  className="name-input"
                  minLength={2}
                  maxLength={20}
                  required
                />
                <button 
                  type="submit" 
                  className="start-button"
                  disabled={!playerName.trim()}
                >
                  Start Playing
                </button>
              </form>
            </div>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="app">
      <div className="game-layout">
        <div className="game-main">
          <div className="game-view">
            <div className="game-header">
              <h2>{gameState.currentMap?.name || 'Loading Map...'}</h2>
              <div className="player-info">
                <span>Playing as: <strong>{gameState.currentUser?.name}</strong></span>
                <span>Players online: {gameState.users.filter(u => u.online).length}</span>
                <span>Maps available: {gameState.maps.length}</span>
                <span>Entities: {gameState.entities.length}</span>
              </div>
            </div>

            {gameState.currentMap ? (
              <div className="map-container">
                <div className="map-info">
                  <p>Map: {gameState.currentMap.name} ({Number(gameState.currentMap.width)}x{Number(gameState.currentMap.height)})</p>
                  <p>Type: {gameState.currentMap.mapType.tag}</p>
                  <p>Starting Town: {gameState.currentMap.isStartingTown ? 'Yes' : 'No'}</p>
                </div>
                
                <div className="map-visual">
                  <div 
                    className="map-grid"
                    style={{
                      display: 'grid',
                      gridTemplateColumns: `repeat(${Number(gameState.currentMap.width)}, 12px)`,
                      gridTemplateRows: `repeat(${Number(gameState.currentMap.height)}, 12px)`,
                      gap: '1px',
                      background: '#333',
                      padding: '4px',
                      borderRadius: '4px',
                      maxWidth: '400px',
                      maxHeight: '400px',
                      overflow: 'auto'
                    }}
                  >
                    {Array.from({ length: Number(gameState.currentMap!.height) }, (_, y) =>
                      Array.from({ length: Number(gameState.currentMap!.width) }, (_, x) => {
                        const currentMap = gameState.currentMap!;
                        const tileIndex = y * Number(currentMap.width) + x;
                        const tileType = currentMap.tiles[tileIndex] || 0;
                        
                        // Check if there's a player at this position
                        const playersAtPosition = gameState.entities.filter(entity => 
                          entity.entityType.tag === 'Player' && 
                          Math.floor(entity.position.x) === x && 
                          Math.floor(entity.position.y) === y
                        );
                        
                        // Check if current player is at this position
                        const isCurrentPlayer = gameState.currentPlayer && 
                          gameState.entities.some(entity => 
                            entity.id === gameState.currentPlayer!.entityId &&
                            Math.floor(entity.position.x) === x && 
                            Math.floor(entity.position.y) === y
                          );
                        
                        let tileColor = '#666'; // Wall (default)
                        if (tileType === 1) tileColor = '#ddd'; // Floor
                        if (tileType === 2) tileColor = '#8B4513'; // Door
                        
                        return (
                          <div
                            key={`${x}-${y}`}
                            className="map-tile"
                            style={{
                              width: '12px',
                              height: '12px',
                              backgroundColor: playersAtPosition.length > 0 
                                ? (isCurrentPlayer ? '#00ff00' : '#ff6b6b') 
                                : tileColor,
                              border: playersAtPosition.length > 0 ? '1px solid #fff' : 'none',
                              cursor: tileType === 1 ? 'pointer' : 'default',
                              position: 'relative'
                            }}
                            onClick={() => {
                              if (tileType === 1) { // Only allow movement to floor tiles
                                movePlayer(x, y);
                              }
                            }}
                            title={`(${x}, ${y}) - ${
                              tileType === 0 ? 'Wall' : 
                              tileType === 1 ? 'Floor' : 
                              tileType === 2 ? 'Door' : 'Unknown'
                            }${playersAtPosition.length > 0 ? ` - ${playersAtPosition.length} player(s)` : ''}`}
                          />
                        );
                      })
                    ).flat()}
                  </div>
                  
                  <div className="map-legend">
                    <div className="legend-item">
                      <div className="legend-color" style={{backgroundColor: '#ddd'}}></div>
                      <span>Floor (walkable)</span>
                    </div>
                    <div className="legend-item">
                      <div className="legend-color" style={{backgroundColor: '#666'}}></div>
                      <span>Wall</span>
                    </div>
                    <div className="legend-item">
                      <div className="legend-color" style={{backgroundColor: '#8B4513'}}></div>
                      <span>Door</span>
                    </div>
                    <div className="legend-item">
                      <div className="legend-color" style={{backgroundColor: '#00ff00'}}></div>
                      <span>You</span>
                    </div>
                    <div className="legend-item">
                      <div className="legend-color" style={{backgroundColor: '#ff6b6b'}}></div>
                      <span>Other Players</span>
                    </div>
                  </div>
                  
                  <p className="map-instructions">Click on floor tiles to move your character</p>
                </div>
              </div>
            ) : (
              <div className="no-map">
                <h3>Loading Map...</h3>
                <p>Waiting for map data from server</p>
              </div>
            )}
          </div>
        </div>
        
        <div className="game-sidebar">
          <div className="chat">
            <div className="chat-header">
              <h3>Chat</h3>
              <span className="player-count">{gameState.users.filter(u => u.online).length} players online</span>
            </div>
            
            <div className="chat-messages">
              {gameState.messages.length === 0 ? (
                <p>No messages yet</p>
              ) : (
                gameState.messages.map((message) => {
                  // Look for sender name in users first, then players
                  const senderUser = gameState.users.find(u => 
                    u.identity.isEqual(message.sender)
                  );
                  const senderPlayer = gameState.players.find(p => 
                    p.identity.isEqual(message.sender)
                  );
                  const senderName = senderUser?.name || senderPlayer?.name || 
                    message.sender.toHexString().substring(0, 8);
                  const isOwnMessage = gameState.currentUser && message.sender.isEqual(gameState.currentUser.identity);
                  
                  return (
                    <div
                      key={message.id.toString()}
                      className={`message ${isOwnMessage ? 'own-message' : ''}`}
                    >
                      <div className="message-header">
                        <span className="sender-name">{senderName}</span>
                        <span className="message-time">
                          {message.sent.toDate().toLocaleTimeString()}
                        </span>
                      </div>
                      <div className="message-text">{message.text}</div>
                    </div>
                  );
                })
              )}
            </div>

            <form 
              onSubmit={(e) => {
                e.preventDefault();
                if (newMessage.trim()) {
                  sendMessage(newMessage.trim());
                  setNewMessage('');
                }
              }} 
              className="chat-input"
            >
              <input
                type="text"
                value={newMessage}
                onChange={(e) => setNewMessage(e.target.value)}
                placeholder="Type a message..."
                className="message-input"
                maxLength={500}
              />
              <button
                type="submit"
                className="send-button"
                disabled={!newMessage.trim()}
              >
                Send
              </button>
            </form>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;