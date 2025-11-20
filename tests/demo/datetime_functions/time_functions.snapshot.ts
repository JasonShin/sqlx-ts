export type NowFunctionParams = [];

export interface INowFunctionResult {
	currentTime: string;
	currentTimeOnly: string;
	id: number;
	name: string;
}

export interface INowFunctionQuery {
	params: NowFunctionParams;
	result: INowFunctionResult;
}

export type TimeComparisonParams = [];

export interface ITimeComparisonResult {
	id: number;
	loginTime: any | null;
	logoutTime: any | null;
	name: string;
}

export interface ITimeComparisonQuery {
	params: TimeComparisonParams;
	result: ITimeComparisonResult;
}

export type IntervalOperationsParams = [];

export interface IIntervalOperationsResult {
	createdAt: any | null;
	id: number;
	name: string;
	oneHourLater: number;
	thirtyMinutesAgo: number;
}

export interface IIntervalOperationsQuery {
	params: IntervalOperationsParams;
	result: IIntervalOperationsResult;
}

export type DateDifferenceParams = [];

export interface IDateDifferenceResult {
	id: number;
	name: string;
	sessionDuration: any | null;
}

export interface IDateDifferenceQuery {
	params: DateDifferenceParams;
	result: IDateDifferenceResult;
}

