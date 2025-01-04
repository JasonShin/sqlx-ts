export type UpdateQueryParams = [string, string | null, number];

export interface IUpdateQueryResult {
	
};

export interface IUpdateQueryQuery {
	params: UpdateQueryParams;
	result: IUpdateQueryResult;
};

export type QuotedUpdateQueryParams = [string, string | null, number];

export interface IQuotedUpdateQueryResult {
	
};

export interface IQuotedUpdateQueryQuery {
	params: QuotedUpdateQueryParams;
	result: IQuotedUpdateQueryResult;
};

export type NullableFieldUpdateParams = [string | null, number];

export interface INullableFieldUpdateResult {
	
};

export interface INullableFieldUpdateQuery {
	params: NullableFieldUpdateParams;
	result: INullableFieldUpdateResult;
};

