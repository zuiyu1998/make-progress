import dayjs from 'dayjs';

export enum DateFormat {
  All = 'YYYY-MM-DD HH:mm:ss',
}

export function dateFormat(
  date?: string,
  formate: DateFormat = DateFormat.All
) {
  return dayjs(date).format(formate);
}
