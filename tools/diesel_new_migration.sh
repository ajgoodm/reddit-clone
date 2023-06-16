if [ -z $1 ]
then
  echo "must provide migration name"
  exit 1
fi

export $(xargs <.env)
export DATABASE_URL=postgres://$RC_POSTGRES_USER@$RC_POSTGRES_HOST:$RC_POSTGRES_PORT/$RC_POSTGRES_DB_NAME
diesel migration generate $1
