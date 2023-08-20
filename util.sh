#/bin/bash
set -e

display_help() {
   # Display Help
   echo "Syntax: ./util.sh [command]"
   echo
   echo "---commands---"
   echo "create-topic           Create a new kafka topic"
   echo "delete-topic           Delete a kafka topic"
   echo "purge-topic            Purge a kafka topic"
   echo
}

create_topic() {
    docker exec -it kafka1 /bin/kafka-topics --bootstrap-server kafka1:29092 --create --topic $1
}

delete_topic() {
    docker exec -it kafka1 /bin/kafka-topics --bootstrap-server kafka1:29092 --delete --topic $1
}

case $1 in

create-topic)
    create_topic $2
;;

delete-topic)
    delete_topic $2
;;

purge-topic)
    delete_topic $2 && create_topic $2
;;

help)
   display_help
;;

*)
   echo "No command specified, displaying help"
   display_help
;;
esac