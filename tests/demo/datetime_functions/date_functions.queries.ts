export type CurrentDateTimestampParams = [];

export interface ICurrentDateTimestampResult {
	id: number;
	name: string;
	now: string;
	today: string;
}

export interface ICurrentDateTimestampQuery {
	params: CurrentDateTimestampParams;
	result: ICurrentDateTimestampResult;
}

export type DateTruncParams = [];

export interface IDateTruncResult {
	createdDay: string;
	createdMonth: string;
	createdYear: string;
	id: number;
	name: string;
}

export interface IDateTruncQuery {
	params: DateTruncParams;
	result: IDateTruncResult;
}

export type ExtractFunctionParams = [];

export interface IExtractFunctionResult {
	createdDay: Date;
	createdMonth: Date;
	createdYear: Date;
	id: number;
	name: string;
}

export interface IExtractFunctionQuery {
	params: ExtractFunctionParams;
	result: IExtractFunctionResult;
}

export type AgeFunctionParams = [];

export interface IAgeFunctionResult {
	accountAge: string;
	accountAgeExplicit: string;
	id: number;
	name: string;
}

export interface IAgeFunctionQuery {
	params: AgeFunctionParams;
	result: IAgeFunctionResult;
}

export type DateArithmeticParams = [];

export interface IDateArithmeticResult {
	createdAt: any | null;
	id: number;
	name: string;
	oneMonthAgo: number;
	oneWeekLater: number;
}

export interface IDateArithmeticQuery {
	params: DateArithmeticParams;
	result: IDateArithmeticResult;
}
