#pragma once

namespace QCAC{

    class RingBufferResizeException : public std::exception {

    };
    class RingBufferRangeException : public std::exception {

    };
    
    template <class T>
    class RingBuffer {
    public:
        RingBuffer(size_t capacity = 1000) :
            m_Capacity(capacity), 
            m_Buffer(std::make_unique<T[]>(capacity)) 
        {};

        void Clear() { m_Size = 0; };

        void Resize(size_t capacity)
        {
            if (capacity < m_Size)
                throw RingBufferResizeException();

            std::unique_ptr<T[]> newBuffer = std::make_unique<T[]>(capacity);

            size_t frontPtr = 0;
            size_t copyElements = m_Size;

            while (copyElements > 0) {
                size_t copyElementsIter = std::min(m_Capacity - m_FrontPtr, copyElements);

                std::memcpy(
                    newBuffer.get() + (frontPtr % capacity),
                    m_Buffer.get() + ((m_FrontPtr + frontPtr) % m_Capacity),
                    copyElementsIter * sizeof(T));

                copyElements -= copyElementsIter;
                frontPtr += copyElementsIter;
            }

            m_FrontPtr = 0;
            m_Capacity = capacity;
            m_Buffer.swap(newBuffer);
        };

        T Front() const { return (*this)[0]; };
        T Back() const { return (*this)[m_Size - 1]; };

        T operator[](size_t index) const 
        { 
            return m_Buffer[GetLocalIndex(index)]; 
        };

        void Add(T element)
        {
            if (IsFull())
                throw RingBufferRangeException();

            m_Size++;
            m_Buffer[GetLocalIndex(m_Size - 1)] = element;
        };

        T PopFront()
        {
            if (IsEmpty())
                throw RingBufferRangeException();

            size_t oldFront = m_FrontPtr;
            m_FrontPtr = (m_FrontPtr + 1) % m_Capacity;
            m_Size--;

            return m_Buffer[oldFront];
        };

        T PopBack()
        {
            if (IsEmpty())
                throw RingBufferRangeException();

            size_t oldBack = GetLocalIndex(m_Size - 1);
            m_Size--;

            return m_Buffer[oldBack];
        };

        size_t Size() const { return m_Size; };
        size_t Capacity() const { return m_Capacity; };
        bool IsEmpty() const { return m_Size == 0; };
        bool IsFull() const { return m_Size == m_Capacity; };

    private:
        size_t m_Size = 0;
        size_t m_Capacity;

        size_t m_FrontPtr = 0;

        std::unique_ptr<T[]> m_Buffer;

        size_t GetLocalIndex(size_t index) const
        {
            if (index >= m_Capacity || index >= m_Size)
                throw RingBufferRangeException();

            return (m_FrontPtr + index) % m_Capacity;
        };
    };

}