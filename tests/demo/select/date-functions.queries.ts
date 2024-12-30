export type AtTimeZoneParams = [];

export interface IAtTimeZoneResult {
	id: number;
	someDate: Date;
};

export interface IAtTimeZoneQuery {
	params: AtTimeZoneParams;
	result: IAtTimeZoneResult;
};

export type CastParams = [];

export interface ICastResult {
	date: Date;
	id: number;
};

export interface ICastQuery {
	params: CastParams;
	result: ICastResult;
};

export type ExtractParams = [];

export interface IExtractResult {
	id: number;
	theMonth: Date;
};

export interface IExtractQuery {
	params: ExtractParams;
	result: IExtractResult;
};
