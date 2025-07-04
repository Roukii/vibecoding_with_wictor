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
export type PlayerOffline = {
  identity: Identity,
  name: string,
  entityId: bigint | undefined,
  currentMapId: bigint | undefined,
  lastSeen: Timestamp,
};

/**
 * A namespace for generated helper functions.
 */
export namespace PlayerOffline {
  /**
  * A function which returns this type represented as an AlgebraicType.
  * This function is derived from the AlgebraicType used to generate this type.
  */
  export function getTypeScriptAlgebraicType(): AlgebraicType {
    return AlgebraicType.createProductType([
      new ProductTypeElement("identity", AlgebraicType.createIdentityType()),
      new ProductTypeElement("name", AlgebraicType.createStringType()),
      new ProductTypeElement("entityId", AlgebraicType.createOptionType(AlgebraicType.createU64Type())),
      new ProductTypeElement("currentMapId", AlgebraicType.createOptionType(AlgebraicType.createU64Type())),
      new ProductTypeElement("lastSeen", AlgebraicType.createTimestampType()),
    ]);
  }

  export function serialize(writer: BinaryWriter, value: PlayerOffline): void {
    PlayerOffline.getTypeScriptAlgebraicType().serialize(writer, value);
  }

  export function deserialize(reader: BinaryReader): PlayerOffline {
    return PlayerOffline.getTypeScriptAlgebraicType().deserialize(reader);
  }

}


