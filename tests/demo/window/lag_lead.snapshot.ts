export type BasicLagParams = [];

export interface IBasicLagResult {
	id: number;
	name: string;
	previousName: string;
}

export interface IBasicLagQuery {
	params: BasicLagParams;
	result: IBasicLagResult;
}

export type BasicLeadParams = [];

export interface IBasicLeadResult {
	id: number;
	name: string;
	nextName: string;
}

export interface IBasicLeadQuery {
	params: BasicLeadParams;
	result: IBasicLeadResult;
}

export type LagWithDefaultParams = [];

export interface ILagWithDefaultResult {
	id: number;
	name: string;
	previousName: string;
}

export interface ILagWithDefaultQuery {
	params: LagWithDefaultParams;
	result: ILagWithDefaultResult;
}

export type LagAndLeadParams = [];

export interface ILagAndLeadResult {
	id: number;
	name: string;
	nextName: string;
	previousName: string;
}

export interface ILagAndLeadQuery {
	params: LagAndLeadParams;
	result: ILagAndLeadResult;
}

export type LagWithPartitionParams = [];

export interface ILagWithPartitionResult {
	id: number;
	name: string;
	previousInRarity: string;
	rarity: string | null;
}

export interface ILagWithPartitionQuery {
	params: LagWithPartitionParams;
	result: ILagWithPartitionResult;
}

export type FirstLastValueParams = [];

export interface IFirstLastValueResult {
	firstInRarity: string;
	id: number;
	lastInRarity: string;
	name: string;
	rarity: string | null;
}

export interface IFirstLastValueQuery {
	params: FirstLastValueParams;
	result: IFirstLastValueResult;
}
