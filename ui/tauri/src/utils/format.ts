import dayjs from 'dayjs';

export enum DateFormat {
  All = 'YYYY-MM-DD HH:mm:ss',
  Year = 'YYYY-MM-DD',
}

export function dateFormat(
  date?: string,
  formate: DateFormat = DateFormat.All
) {
  let instance = dayjs(date);

  if (!instance.isValid()) {
    instance = dayjs();
  }

  return instance.format(formate);
}

export function timestamp(date?: string) {
  return dayjs(date).valueOf();
}
