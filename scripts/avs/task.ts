export type Task = {
  payload: {
    user: string;
    action: string;
    amount: number;
  };
};

export type TaskResult = {
  output: string;
};
