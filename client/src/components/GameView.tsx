import React, { useCallback, useMemo } from 'react';
import { Player, Map, Entity } from '../module_bindings';

interface GameViewProps {
  currentPlayer: Player;
  players: Player[];
  currentMap: Map | null;
  entities: Entity[];
  onMovePlayer: (x: number, y: number) => Promise<void>;
}

const GameView: React.FC<GameViewProps> = ({
  currentPlayer,
  players,
  currentMap,
  entities,
  onMovePlayer
}) => {
  // Get current player's entity position
  const currentPlayerEntity = useMemo(() => {
    if (!currentPlayer.entityId) return null;
    return entities.find(e => e.id === currentPlayer.entityId);
  }, [currentPlayer.entityId, entities]);

  const handleTileClick = useCallback((x: number, y: number) => {
    onMovePlayer(x, y);
  }, [onMovePlayer]);

  const getTileType = useCallback((tileValue: number) => {
    // Map tile values to CSS classes
    switch (tileValue) {
      case 0: return 'wall';
      case 1: return 'floor';
      case 2: return 'door';
      case 3: return 'water';
      case 4: return 'grass';
      default: return 'unknown';
    }
  }, []);

  const getEntityAtPosition = useCallback((x: number, y: number) => {
    return entities.find(e => 
      Math.floor(e.position.x) === x && Math.floor(e.position.y) === y
    );
  }, [entities]);

  const getPlayerAtPosition = useCallback((x: number, y: number) => {
    const entity = getEntityAtPosition(x, y);
    if (!entity) return null;
    return players.find(p => p.entityId === entity.id);
  }, [getEntityAtPosition, players]);

  if (!currentMap) {
    return (
      <div className="game-view">
        <div className="no-map">
          <h2>Loading Map...</h2>
          <p>Waiting for map data from server</p>
        </div>
      </div>
    );
  }

  const mapWidth = Number(currentMap.width);
  const mapHeight = Number(currentMap.height);

  return (
    <div className="game-view">
      <div className="game-header">
        <h2>{currentMap.name}</h2>
        <div className="player-info">
          <span>Playing as: <strong>{currentPlayer.name}</strong></span>
          {currentPlayerEntity && (
            <span>
              Position: ({Math.floor(currentPlayerEntity.position.x)}, {Math.floor(currentPlayerEntity.position.y)})
            </span>
          )}
        </div>
      </div>

      <div 
        className="game-map"
        style={{
          gridTemplateColumns: `repeat(${mapWidth}, 1fr)`,
          gridTemplateRows: `repeat(${mapHeight}, 1fr)`
        }}
      >
        {Array.from({ length: mapHeight }, (_, y) =>
          Array.from({ length: mapWidth }, (_, x) => {
            const tileIndex = y * mapWidth + x;
            const tileValue = currentMap.tiles[tileIndex] || 0;
            const tileType = getTileType(tileValue);
            const entityAtPos = getEntityAtPosition(x, y);
            const playerAtPos = getPlayerAtPosition(x, y);
            const isCurrentPlayer = playerAtPos?.identity.isEqual(currentPlayer.identity);

            return (
              <div
                key={`${x}-${y}`}
                className={`tile ${tileType} ${entityAtPos ? 'has-entity' : ''} ${isCurrentPlayer ? 'current-player' : ''}`}
                onClick={() => handleTileClick(x, y)}
                title={`(${x}, ${y}) - ${tileType}${playerAtPos ? ` - ${playerAtPos.name}` : ''}`}
              >
                {playerAtPos && (
                  <div className={`player-marker ${isCurrentPlayer ? 'current' : 'other'}`}>
                    {playerAtPos.name.charAt(0).toUpperCase()}
                  </div>
                )}
                {entityAtPos && !playerAtPos && (
                  <div className="entity-marker">
                    E
                  </div>
                )}
              </div>
            );
          })
        )}
      </div>

      <div className="game-controls">
        <div className="control-hint">
          <p>Click on tiles to move your character</p>
          <div className="legend">
            <div className="legend-item">
              <div className="tile wall small"></div>
              <span>Wall</span>
            </div>
            <div className="legend-item">
              <div className="tile floor small"></div>
              <span>Floor</span>
            </div>
            <div className="legend-item">
              <div className="tile door small"></div>
              <span>Door</span>
            </div>
            <div className="legend-item">
              <div className="player-marker current small">P</div>
              <span>You</span>
            </div>
            <div className="legend-item">
              <div className="player-marker other small">P</div>
              <span>Others</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default GameView; 