// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

/* eslint-disable */
/* tslint:disable */
// @ts-nocheck
import {
  AlgebraicType,
  AlgebraicValue,
  BinaryReader,
  BinaryWriter,
  CallReducerFlags,
  ConnectionId,
  DbConnectionBuilder,
  DbConnectionImpl,
  DbContext,
  ErrorContextInterface,
  Event,
  EventContextInterface,
  Identity,
  ProductType,
  ProductTypeElement,
  ReducerEventContextInterface,
  SubscriptionBuilderImpl,
  SubscriptionEventContextInterface,
  SumType,
  SumTypeVariant,
  TableCache,
  TimeDuration,
  Timestamp,
  deepEqual,
} from "@clockworklabs/spacetimedb-sdk";

// Import and reexport all reducer arg types
import { ClientConnected } from "./client_connected_reducer.ts";
export { ClientConnected };
import { CreatePlayerEntity } from "./create_player_entity_reducer.ts";
export { CreatePlayerEntity };
import { DeleteMessage } from "./delete_message_reducer.ts";
export { DeleteMessage };
import { GetLatestDungeon } from "./get_latest_dungeon_reducer.ts";
export { GetLatestDungeon };
import { GetStartingTown } from "./get_starting_town_reducer.ts";
export { GetStartingTown };
import { IdentityDisconnected } from "./identity_disconnected_reducer.ts";
export { IdentityDisconnected };
import { InitializeGameInfo } from "./initialize_game_info_reducer.ts";
export { InitializeGameInfo };
import { MovePlayer } from "./move_player_reducer.ts";
export { MovePlayer };
import { SendMessage } from "./send_message_reducer.ts";
export { SendMessage };
import { SetName } from "./set_name_reducer.ts";
export { SetName };
import { SpawnPlayerEntity } from "./spawn_player_entity_reducer.ts";
export { SpawnPlayerEntity };
import { Tick } from "./tick_reducer.ts";
export { Tick };

// Import and reexport all table handle types
import { EntityTableHandle } from "./entity_table.ts";
export { EntityTableHandle };
import { GameInfoTableHandle } from "./game_info_table.ts";
export { GameInfoTableHandle };
import { GameTickTableHandle } from "./game_tick_table.ts";
export { GameTickTableHandle };
import { MapTableHandle } from "./map_table.ts";
export { MapTableHandle };
import { MessageTableHandle } from "./message_table.ts";
export { MessageTableHandle };
import { PlayerTableHandle } from "./player_table.ts";
export { PlayerTableHandle };
import { PlayerOfflineTableHandle } from "./player_offline_table.ts";
export { PlayerOfflineTableHandle };
import { UserTableHandle } from "./user_table.ts";
export { UserTableHandle };

// Import and reexport all types
import { Entity } from "./entity_type.ts";
export { Entity };
import { EntityType } from "./entity_type_type.ts";
export { EntityType };
import { GameInfo } from "./game_info_type.ts";
export { GameInfo };
import { GameTick } from "./game_tick_type.ts";
export { GameTick };
import { Map } from "./map_type.ts";
export { Map };
import { MapType } from "./map_type_type.ts";
export { MapType };
import { Message } from "./message_type.ts";
export { Message };
import { Player } from "./player_type.ts";
export { Player };
import { PlayerOffline } from "./player_offline_type.ts";
export { PlayerOffline };
import { User } from "./user_type.ts";
export { User };
import { Vec2 } from "./vec_2_type.ts";
export { Vec2 };

const REMOTE_MODULE = {
  tables: {
    entity: {
      tableName: "entity",
      rowType: Entity.getTypeScriptAlgebraicType(),
      primaryKey: "id",
    },
    game_info: {
      tableName: "game_info",
      rowType: GameInfo.getTypeScriptAlgebraicType(),
      primaryKey: "id",
    },
    game_tick: {
      tableName: "game_tick",
      rowType: GameTick.getTypeScriptAlgebraicType(),
      primaryKey: "id",
    },
    map: {
      tableName: "map",
      rowType: Map.getTypeScriptAlgebraicType(),
      primaryKey: "id",
    },
    message: {
      tableName: "message",
      rowType: Message.getTypeScriptAlgebraicType(),
      primaryKey: "id",
    },
    player: {
      tableName: "player",
      rowType: Player.getTypeScriptAlgebraicType(),
      primaryKey: "identity",
    },
    player_offline: {
      tableName: "player_offline",
      rowType: PlayerOffline.getTypeScriptAlgebraicType(),
      primaryKey: "identity",
    },
    user: {
      tableName: "user",
      rowType: User.getTypeScriptAlgebraicType(),
      primaryKey: "identity",
    },
  },
  reducers: {
    client_connected: {
      reducerName: "client_connected",
      argsType: ClientConnected.getTypeScriptAlgebraicType(),
    },
    create_player_entity: {
      reducerName: "create_player_entity",
      argsType: CreatePlayerEntity.getTypeScriptAlgebraicType(),
    },
    delete_message: {
      reducerName: "delete_message",
      argsType: DeleteMessage.getTypeScriptAlgebraicType(),
    },
    get_latest_dungeon: {
      reducerName: "get_latest_dungeon",
      argsType: GetLatestDungeon.getTypeScriptAlgebraicType(),
    },
    get_starting_town: {
      reducerName: "get_starting_town",
      argsType: GetStartingTown.getTypeScriptAlgebraicType(),
    },
    identity_disconnected: {
      reducerName: "identity_disconnected",
      argsType: IdentityDisconnected.getTypeScriptAlgebraicType(),
    },
    initialize_game_info: {
      reducerName: "initialize_game_info",
      argsType: InitializeGameInfo.getTypeScriptAlgebraicType(),
    },
    move_player: {
      reducerName: "move_player",
      argsType: MovePlayer.getTypeScriptAlgebraicType(),
    },
    send_message: {
      reducerName: "send_message",
      argsType: SendMessage.getTypeScriptAlgebraicType(),
    },
    set_name: {
      reducerName: "set_name",
      argsType: SetName.getTypeScriptAlgebraicType(),
    },
    spawn_player_entity: {
      reducerName: "spawn_player_entity",
      argsType: SpawnPlayerEntity.getTypeScriptAlgebraicType(),
    },
    tick: {
      reducerName: "tick",
      argsType: Tick.getTypeScriptAlgebraicType(),
    },
  },
  // Constructors which are used by the DbConnectionImpl to
  // extract type information from the generated RemoteModule.
  //
  // NOTE: This is not strictly necessary for `eventContextConstructor` because
  // all we do is build a TypeScript object which we could have done inside the
  // SDK, but if in the future we wanted to create a class this would be
  // necessary because classes have methods, so we'll keep it.
  eventContextConstructor: (imp: DbConnectionImpl, event: Event<Reducer>) => {
    return {
      ...(imp as DbConnection),
      event
    }
  },
  dbViewConstructor: (imp: DbConnectionImpl) => {
    return new RemoteTables(imp);
  },
  reducersConstructor: (imp: DbConnectionImpl, setReducerFlags: SetReducerFlags) => {
    return new RemoteReducers(imp, setReducerFlags);
  },
  setReducerFlagsConstructor: () => {
    return new SetReducerFlags();
  }
}

// A type representing all the possible variants of a reducer.
export type Reducer = never
| { name: "ClientConnected", args: ClientConnected }
| { name: "CreatePlayerEntity", args: CreatePlayerEntity }
| { name: "DeleteMessage", args: DeleteMessage }
| { name: "GetLatestDungeon", args: GetLatestDungeon }
| { name: "GetStartingTown", args: GetStartingTown }
| { name: "IdentityDisconnected", args: IdentityDisconnected }
| { name: "InitializeGameInfo", args: InitializeGameInfo }
| { name: "MovePlayer", args: MovePlayer }
| { name: "SendMessage", args: SendMessage }
| { name: "SetName", args: SetName }
| { name: "SpawnPlayerEntity", args: SpawnPlayerEntity }
| { name: "Tick", args: Tick }
;

export class RemoteReducers {
  constructor(private connection: DbConnectionImpl, private setCallReducerFlags: SetReducerFlags) {}

  onClientConnected(callback: (ctx: ReducerEventContext) => void) {
    this.connection.onReducer("client_connected", callback);
  }

  removeOnClientConnected(callback: (ctx: ReducerEventContext) => void) {
    this.connection.offReducer("client_connected", callback);
  }

  createPlayerEntity() {
    this.connection.callReducer("create_player_entity", new Uint8Array(0), this.setCallReducerFlags.createPlayerEntityFlags);
  }

  onCreatePlayerEntity(callback: (ctx: ReducerEventContext) => void) {
    this.connection.onReducer("create_player_entity", callback);
  }

  removeOnCreatePlayerEntity(callback: (ctx: ReducerEventContext) => void) {
    this.connection.offReducer("create_player_entity", callback);
  }

  deleteMessage(messageId: bigint) {
    const __args = { messageId };
    let __writer = new BinaryWriter(1024);
    DeleteMessage.getTypeScriptAlgebraicType().serialize(__writer, __args);
    let __argsBuffer = __writer.getBuffer();
    this.connection.callReducer("delete_message", __argsBuffer, this.setCallReducerFlags.deleteMessageFlags);
  }

  onDeleteMessage(callback: (ctx: ReducerEventContext, messageId: bigint) => void) {
    this.connection.onReducer("delete_message", callback);
  }

  removeOnDeleteMessage(callback: (ctx: ReducerEventContext, messageId: bigint) => void) {
    this.connection.offReducer("delete_message", callback);
  }

  getLatestDungeon() {
    this.connection.callReducer("get_latest_dungeon", new Uint8Array(0), this.setCallReducerFlags.getLatestDungeonFlags);
  }

  onGetLatestDungeon(callback: (ctx: ReducerEventContext) => void) {
    this.connection.onReducer("get_latest_dungeon", callback);
  }

  removeOnGetLatestDungeon(callback: (ctx: ReducerEventContext) => void) {
    this.connection.offReducer("get_latest_dungeon", callback);
  }

  getStartingTown() {
    this.connection.callReducer("get_starting_town", new Uint8Array(0), this.setCallReducerFlags.getStartingTownFlags);
  }

  onGetStartingTown(callback: (ctx: ReducerEventContext) => void) {
    this.connection.onReducer("get_starting_town", callback);
  }

  removeOnGetStartingTown(callback: (ctx: ReducerEventContext) => void) {
    this.connection.offReducer("get_starting_town", callback);
  }

  onIdentityDisconnected(callback: (ctx: ReducerEventContext) => void) {
    this.connection.onReducer("identity_disconnected", callback);
  }

  removeOnIdentityDisconnected(callback: (ctx: ReducerEventContext) => void) {
    this.connection.offReducer("identity_disconnected", callback);
  }

  initializeGameInfo(startingTownMapId: bigint) {
    const __args = { startingTownMapId };
    let __writer = new BinaryWriter(1024);
    InitializeGameInfo.getTypeScriptAlgebraicType().serialize(__writer, __args);
    let __argsBuffer = __writer.getBuffer();
    this.connection.callReducer("initialize_game_info", __argsBuffer, this.setCallReducerFlags.initializeGameInfoFlags);
  }

  onInitializeGameInfo(callback: (ctx: ReducerEventContext, startingTownMapId: bigint) => void) {
    this.connection.onReducer("initialize_game_info", callback);
  }

  removeOnInitializeGameInfo(callback: (ctx: ReducerEventContext, startingTownMapId: bigint) => void) {
    this.connection.offReducer("initialize_game_info", callback);
  }

  movePlayer(x: number, y: number) {
    const __args = { x, y };
    let __writer = new BinaryWriter(1024);
    MovePlayer.getTypeScriptAlgebraicType().serialize(__writer, __args);
    let __argsBuffer = __writer.getBuffer();
    this.connection.callReducer("move_player", __argsBuffer, this.setCallReducerFlags.movePlayerFlags);
  }

  onMovePlayer(callback: (ctx: ReducerEventContext, x: number, y: number) => void) {
    this.connection.onReducer("move_player", callback);
  }

  removeOnMovePlayer(callback: (ctx: ReducerEventContext, x: number, y: number) => void) {
    this.connection.offReducer("move_player", callback);
  }

  sendMessage(text: string) {
    const __args = { text };
    let __writer = new BinaryWriter(1024);
    SendMessage.getTypeScriptAlgebraicType().serialize(__writer, __args);
    let __argsBuffer = __writer.getBuffer();
    this.connection.callReducer("send_message", __argsBuffer, this.setCallReducerFlags.sendMessageFlags);
  }

  onSendMessage(callback: (ctx: ReducerEventContext, text: string) => void) {
    this.connection.onReducer("send_message", callback);
  }

  removeOnSendMessage(callback: (ctx: ReducerEventContext, text: string) => void) {
    this.connection.offReducer("send_message", callback);
  }

  setName(name: string) {
    const __args = { name };
    let __writer = new BinaryWriter(1024);
    SetName.getTypeScriptAlgebraicType().serialize(__writer, __args);
    let __argsBuffer = __writer.getBuffer();
    this.connection.callReducer("set_name", __argsBuffer, this.setCallReducerFlags.setNameFlags);
  }

  onSetName(callback: (ctx: ReducerEventContext, name: string) => void) {
    this.connection.onReducer("set_name", callback);
  }

  removeOnSetName(callback: (ctx: ReducerEventContext, name: string) => void) {
    this.connection.offReducer("set_name", callback);
  }

  spawnPlayerEntity() {
    this.connection.callReducer("spawn_player_entity", new Uint8Array(0), this.setCallReducerFlags.spawnPlayerEntityFlags);
  }

  onSpawnPlayerEntity(callback: (ctx: ReducerEventContext) => void) {
    this.connection.onReducer("spawn_player_entity", callback);
  }

  removeOnSpawnPlayerEntity(callback: (ctx: ReducerEventContext) => void) {
    this.connection.offReducer("spawn_player_entity", callback);
  }

  tick(schedule: GameTick) {
    const __args = { schedule };
    let __writer = new BinaryWriter(1024);
    Tick.getTypeScriptAlgebraicType().serialize(__writer, __args);
    let __argsBuffer = __writer.getBuffer();
    this.connection.callReducer("tick", __argsBuffer, this.setCallReducerFlags.tickFlags);
  }

  onTick(callback: (ctx: ReducerEventContext, schedule: GameTick) => void) {
    this.connection.onReducer("tick", callback);
  }

  removeOnTick(callback: (ctx: ReducerEventContext, schedule: GameTick) => void) {
    this.connection.offReducer("tick", callback);
  }

}

export class SetReducerFlags {
  createPlayerEntityFlags: CallReducerFlags = 'FullUpdate';
  createPlayerEntity(flags: CallReducerFlags) {
    this.createPlayerEntityFlags = flags;
  }

  deleteMessageFlags: CallReducerFlags = 'FullUpdate';
  deleteMessage(flags: CallReducerFlags) {
    this.deleteMessageFlags = flags;
  }

  getLatestDungeonFlags: CallReducerFlags = 'FullUpdate';
  getLatestDungeon(flags: CallReducerFlags) {
    this.getLatestDungeonFlags = flags;
  }

  getStartingTownFlags: CallReducerFlags = 'FullUpdate';
  getStartingTown(flags: CallReducerFlags) {
    this.getStartingTownFlags = flags;
  }

  initializeGameInfoFlags: CallReducerFlags = 'FullUpdate';
  initializeGameInfo(flags: CallReducerFlags) {
    this.initializeGameInfoFlags = flags;
  }

  movePlayerFlags: CallReducerFlags = 'FullUpdate';
  movePlayer(flags: CallReducerFlags) {
    this.movePlayerFlags = flags;
  }

  sendMessageFlags: CallReducerFlags = 'FullUpdate';
  sendMessage(flags: CallReducerFlags) {
    this.sendMessageFlags = flags;
  }

  setNameFlags: CallReducerFlags = 'FullUpdate';
  setName(flags: CallReducerFlags) {
    this.setNameFlags = flags;
  }

  spawnPlayerEntityFlags: CallReducerFlags = 'FullUpdate';
  spawnPlayerEntity(flags: CallReducerFlags) {
    this.spawnPlayerEntityFlags = flags;
  }

  tickFlags: CallReducerFlags = 'FullUpdate';
  tick(flags: CallReducerFlags) {
    this.tickFlags = flags;
  }

}

export class RemoteTables {
  constructor(private connection: DbConnectionImpl) {}

  get entity(): EntityTableHandle {
    return new EntityTableHandle(this.connection.clientCache.getOrCreateTable<Entity>(REMOTE_MODULE.tables.entity));
  }

  get gameInfo(): GameInfoTableHandle {
    return new GameInfoTableHandle(this.connection.clientCache.getOrCreateTable<GameInfo>(REMOTE_MODULE.tables.game_info));
  }

  get gameTick(): GameTickTableHandle {
    return new GameTickTableHandle(this.connection.clientCache.getOrCreateTable<GameTick>(REMOTE_MODULE.tables.game_tick));
  }

  get map(): MapTableHandle {
    return new MapTableHandle(this.connection.clientCache.getOrCreateTable<Map>(REMOTE_MODULE.tables.map));
  }

  get message(): MessageTableHandle {
    return new MessageTableHandle(this.connection.clientCache.getOrCreateTable<Message>(REMOTE_MODULE.tables.message));
  }

  get player(): PlayerTableHandle {
    return new PlayerTableHandle(this.connection.clientCache.getOrCreateTable<Player>(REMOTE_MODULE.tables.player));
  }

  get playerOffline(): PlayerOfflineTableHandle {
    return new PlayerOfflineTableHandle(this.connection.clientCache.getOrCreateTable<PlayerOffline>(REMOTE_MODULE.tables.player_offline));
  }

  get user(): UserTableHandle {
    return new UserTableHandle(this.connection.clientCache.getOrCreateTable<User>(REMOTE_MODULE.tables.user));
  }
}

export class SubscriptionBuilder extends SubscriptionBuilderImpl<RemoteTables, RemoteReducers, SetReducerFlags> { }

export class DbConnection extends DbConnectionImpl<RemoteTables, RemoteReducers, SetReducerFlags> {
  static builder = (): DbConnectionBuilder<DbConnection, ErrorContext, SubscriptionEventContext> => {
    return new DbConnectionBuilder<DbConnection, ErrorContext, SubscriptionEventContext>(REMOTE_MODULE, (imp: DbConnectionImpl) => imp as DbConnection);
  }
  subscriptionBuilder = (): SubscriptionBuilder => {
    return new SubscriptionBuilder(this);
  }
}

export type EventContext = EventContextInterface<RemoteTables, RemoteReducers, SetReducerFlags, Reducer>;
export type ReducerEventContext = ReducerEventContextInterface<RemoteTables, RemoteReducers, SetReducerFlags, Reducer>;
export type SubscriptionEventContext = SubscriptionEventContextInterface<RemoteTables, RemoteReducers, SetReducerFlags>;
export type ErrorContext = ErrorContextInterface<RemoteTables, RemoteReducers, SetReducerFlags>;
