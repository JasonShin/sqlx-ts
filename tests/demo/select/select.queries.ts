export type SelectSql1Params = [];

export interface ISelectSql1Result {
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	rarity: string | null;
}

export interface ISelectSql1Query {
	params: SelectSql1Params;
	result: ISelectSql1Result;
}

export type SelectSql2Params = [];

export interface ISelectSql2Result {
	character_id: number | null;
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	quantity: number | null;
	rarity: string | null;
}

export interface ISelectSql2Query {
	params: SelectSql2Params;
	result: ISelectSql2Result;
}

export type SelectSql3Params = [];

export interface ISelectSql3Result {
	inventoryQuantity: number | null;
}

export interface ISelectSql3Query {
	params: SelectSql3Params;
	result: ISelectSql3Result;
}

export type SelectSql4Params = [];

export interface ISelectSql4Result {
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	rarity: string | null;
}

export interface ISelectSql4Query {
	params: SelectSql4Params;
	result: ISelectSql4Result;
}

export type SelectSql5Params = [];

export interface ISelectSql5Result {
	character_id: number | null;
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	quantity: number | null;
	rarity: string | null;
}

export interface ISelectSql5Query {
	params: SelectSql5Params;
	result: ISelectSql5Result;
}

export type SelectSql6Params = [number | null, number | null];

export interface ISelectSql6Result {
	id: number;
}

export interface ISelectSql6Query {
	params: SelectSql6Params;
	result: ISelectSql6Result;
}

export type SelectSql9Params = [any];

export interface ISelectSql9Result {
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	rarity: string | null;
}

export interface ISelectSql9Query {
	params: SelectSql9Params;
	result: ISelectSql9Result;
}

export type SelectSql10Params = [];

export interface ISelectSql10Result {
	completed: boolean | null;
	description: string | null;
	id: number;
	name: string;
	required_level: number | null;
	rewards: object | null;
}

export interface ISelectSql10Query {
	params: SelectSql10Params;
	result: ISelectSql10Result;
}

export type SelectSql11Params = [string, string];

export interface ISelectSql11Result {
	hmm: any;
	id: number;
	quantity: number | null;
}

export interface ISelectSql11Query {
	params: SelectSql11Params;
	result: ISelectSql11Result;
}

export type SelectSql12Params = [number];

export interface ISelectSql12Result {
	id: number;
}

export interface ISelectSql12Query {
	params: SelectSql12Params;
	result: ISelectSql12Result;
}

export type SelectSql13Params = [string];

export interface ISelectSql13Result {
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	rarity: string | null;
}

export interface ISelectSql13Query {
	params: SelectSql13Params;
	result: ISelectSql13Result;
}

export type SelectSql14Params = [number];

export interface ISelectSql14Result {
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	rarity: string | null;
}

export interface ISelectSql14Query {
	params: SelectSql14Params;
	result: ISelectSql14Result;
}

export type SelectSql15Params = [];

export interface ISelectSql15Result {
	id2: number;
	inventoryId: number;
	itemsId: number;
}

export interface ISelectSql15Query {
	params: SelectSql15Params;
	result: ISelectSql15Result;
}

export type SelectSql16Params = [];

export interface ISelectSql16Result {
	name: 'alliance' | 'horde';
}

export interface ISelectSql16Query {
	params: SelectSql16Params;
	result: ISelectSql16Result;
}

export type SelectSql17Params = [];

export interface ISelectSql17Result {
	itemRarity: string;
}

export interface ISelectSql17Query {
	params: SelectSql17Params;
	result: ISelectSql17Result;
}

export type SelectSql18Params = [];

export interface ISelectSql18Result {
	qty: number;
}

export interface ISelectSql18Query {
	params: SelectSql18Params;
	result: ISelectSql18Result;
}

export type SelectSql19Params = [];

export interface ISelectSql19Result {
	characterLevel: number;
}

export interface ISelectSql19Query {
	params: SelectSql19Params;
	result: ISelectSql19Result;
}

export type SelectSql20Params = [];

export interface ISelectSql20Result {
	goldAmount: number;
}

export interface ISelectSql20Query {
	params: SelectSql20Params;
	result: ISelectSql20Result;
}

export type SelectSql21Params = [];

export interface ISelectSql21Result {
	desc: string;
}

export interface ISelectSql21Query {
	params: SelectSql21Params;
	result: ISelectSql21Result;
}

export type SelectSql22Params = [];

export interface ISelectSql22Result {
	itemInfo: string;
}

export interface ISelectSql22Query {
	params: SelectSql22Params;
	result: ISelectSql22Result;
}

export type SelectSql23Params = [];

export interface ISelectSql23Result {
	charName: string;
}

export interface ISelectSql23Query {
	params: SelectSql23Params;
	result: ISelectSql23Result;
}

export type SelectSql24Params = [];

export interface ISelectSql24Result {
	questLevel: number;
}

export interface ISelectSql24Query {
	params: SelectSql24Params;
	result: ISelectSql24Result;
}

export type SelectSql25Params = [];

export interface ISelectSql25Result {
	guildName: string;
}

export interface ISelectSql25Query {
	params: SelectSql25Params;
	result: ISelectSql25Result;
}

export type SelectSql26Params = [];

export interface ISelectSql26Result {
	invQuantity: number;
}

export interface ISelectSql26Query {
	params: SelectSql26Params;
	result: ISelectSql26Result;
}

export type SelectSql27Params = [];

export interface ISelectSql27Result {
	rank: string;
}

export interface ISelectSql27Query {
	params: SelectSql27Params;
	result: ISelectSql27Result;
}

export type SelectSql28Params = [];

export interface ISelectSql28Result {
	itemName: string;
}

export interface ISelectSql28Query {
	params: SelectSql28Params;
	result: ISelectSql28Result;
}

export type SelectSql29Params = [];

export interface ISelectSql29Result {
	itemName: string;
	qty: number;
}

export interface ISelectSql29Query {
	params: SelectSql29Params;
	result: ISelectSql29Result;
}

export type SelectSql30Params = [];

export interface ISelectSql30Result {
	charName: string;
	lvl: number;
	nonZeroGold: number;
}

export interface ISelectSql30Query {
	params: SelectSql30Params;
	result: ISelectSql30Result;
}
