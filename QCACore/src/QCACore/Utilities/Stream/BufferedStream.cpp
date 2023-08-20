#include "BufferedStream.h"

namespace QCAC {

	template<class T>
	inline size_t BufferedStream<T>::Seek(int64_t offset, SeekOrigin origin)
	{
		return size_t();
	}

	template<class T>
	size_t BufferedStream<T>::Read(T* buffer, size_t count)
	{
		return size_t();
	}

	template<class T>
	size_t BufferedStream<T>::Write(T* buffer, size_t count)
	{
		return size_t();
	}

}