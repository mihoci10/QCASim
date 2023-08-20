#pragma once

#include <QCACore/Data/IStream.hpp>

namespace QCAC {

	template <class T>
	class BufferedStream : public IStream<T> {
	public:
		size_t Seek(int64_t offset, SeekOrigin origin) override;

		size_t Read(T* buffer, size_t count) override;
		size_t Write(T* buffer, size_t count) override;

	private:
		std::unique_ptr<T> m_Buffer;

	};

}