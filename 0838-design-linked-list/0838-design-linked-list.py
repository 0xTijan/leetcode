class Node:
    def __init__(self, val):
        self.val = val
        self.next = None

class MyLinkedList:
    def __init__(self):
        self.head = None
        self.tail = None

    def get(self, index: int) -> int:
        cur = self.head
        for i in range(index):
            if cur is None:
                return -1
            cur = cur.next
        return cur.val if cur else -1

    def addAtHead(self, val: int):
        new_node = Node(val)
        new_node.next = self.head  # Point to the current head
        self.head = new_node  # Set new node as head
        
        # If the list was empty, set the tail as well
        if self.tail is None:
            self.tail = new_node

    def addAtTail(self, val: int):
        new_node = Node(val)
        
        if self.tail is not None:
            self.tail.next = new_node  # Link new node to the current tail
        else:
            self.head = new_node  # If list was empty, set head to new node
            
        self.tail = new_node  # Update the tail

    def addAtIndex(self, index: int, val: int):
        if index == 0:
            self.addAtHead(val)
            return

        cur = self.head
        for i in range(index - 1):
            if cur is None:
                return  # Index is out of bounds
            cur = cur.next
            
        if cur is not None:  # We found the node before the index
            new_node = Node(val)
            new_node.next = cur.next  # Link new node to the next node
            cur.next = new_node  # Link current node to new node
            
            # If new node is added at the end, update the tail
            if new_node.next is None:
                self.tail = new_node

    def deleteAtIndex(self, index: int):
        if index < 0:
            return
        
        if index == 0:  # Deleting the head
            if self.head is not None:
                self.head = self.head.next  # Move head to next node
                # If head is now None, the list is empty; set tail to None
                if self.head is None:
                    self.tail = None
            return

        cur = self.head
        for i in range(index - 1):
            if cur is None:
                return  # Index is out of bounds
            cur = cur.next
            
        if cur and cur.next:  # We found the node before the one to delete
            cur.next = cur.next.next  # Skip the node to delete
            
            # If we just deleted the tail, update the tail
            if cur.next is None:
                self.tail = cur