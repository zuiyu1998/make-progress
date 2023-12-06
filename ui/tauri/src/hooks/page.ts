import React from 'react';

export function usePageConfig(initPage = 0, initPageSize = 50) {
  const [hasNext, setHasNext] = React.useState(true);
  const [page, setPage] = React.useState(initPage);
  const [pageSize, setPageSize] = React.useState(initPageSize);

  const [loading, setLoading] = React.useState(false);

  return {
    page,
    setPage,
    hasNext,
    setHasNext,
    pageSize,
    setPageSize,
    loading,
    setLoading,
  };
}
